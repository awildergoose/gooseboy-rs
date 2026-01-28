// run this with 640000 initial memory and 1280000 maximum memory and
// with the -Xmx4G java flag
#![allow(clippy::similar_names)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(static_mut_refs)]
#![no_main]

use std::{cell::RefCell, rc::Rc};

use gooseboy::{
    color::Color,
    framebuffer::{clear_framebuffer, get_framebuffer_height, get_framebuffer_width, init_fb},
    keys::{
        KEY_A, KEY_BACKSPACE, KEY_DELETE, KEY_DOWN, KEY_END, KEY_ENTER, KEY_ESCAPE, KEY_HOME,
        KEY_INSERT, KEY_LEFT, KEY_PAGE_DOWN, KEY_PAGE_UP, KEY_RIGHT, KEY_TAB, KEY_UP, KEY_Z,
    },
    log,
    text::draw_text_wrapped_ex,
};

use rv64emu::{
    config::Config,
    device::{
        device_16550a::Device16550aUART, device_memory::DeviceMemory,
        device_sifive_plic::SIFIVE_UART_IRQ, device_sifive_uart::DeviceSifiveUart,
        device_trait::MEM_BASE,
    },
    rv64core::{
        bus::{Bus, DeviceType},
        cpu_core::CpuCoreBuild,
    },
    rvsim::RVsim,
    tools::{FifoUnbounded, rc_refcell_new},
};

use crate::ansi::{AnsiCode, AnsiParser, AnsiState};
use crossbeam_queue::SegQueue;

mod ansi;

const HELLO_BIN: &[u8] = include_bytes!("../linux.elf");

#[derive(Clone, Copy)]
struct ConsoleCell {
    ch: char,
    color: Color,
}

static mut SIM: Option<RVsim> = None;
static mut UART_TX: Option<FifoUnbounded<u8>> = None;
static mut UART_RX: Option<FifoUnbounded<u8>> = None;
static mut CONSOLE_LINES: Vec<Vec<ConsoleCell>> = Vec::new();
static mut CUR_Y: usize = 0;
static mut ANSI_PARSER: AnsiParser = AnsiParser {
    state: AnsiState::Normal,
    buffer: Vec::new(),
};
static mut CURRENT_COLOR: Color = Color::WHITE;
static mut CURRENT_BOLD: bool = false;

#[gooseboy::main]
fn main() {
    init_fb();

    let mut config = Config::new();
    config.set_mmu_type("sv39");
    config.set_isa("rv64imac");
    config.set_s_mode();
    let config = Rc::new(config);

    let bus = rc_refcell_new(Bus::new());
    let mem = DeviceMemory::new(128 * 1024 * 1024);
    bus.borrow_mut().add_device(DeviceType {
        start: MEM_BASE,
        len: mem.size() as u64,
        instance: Box::new(mem),
        name: "RAM",
    });

    let uart_tx_fifo = FifoUnbounded::new(SegQueue::<u8>::new());
    let uart_rx_fifo = FifoUnbounded::new(SegQueue::<u8>::new());

    let device_16550 = Device16550aUART::new(uart_tx_fifo.clone(), uart_rx_fifo.clone());
    bus.borrow_mut().add_device(DeviceType {
        start: 0x1000_0000,
        len: 0x1000,
        instance: Box::new(device_16550),
        name: "16550a_uart",
    });

    let device_sifive = DeviceSifiveUart::new(uart_tx_fifo.clone(), uart_rx_fifo.clone());
    bus.borrow_mut()
        .plic
        .instance
        .register_irq_source(SIFIVE_UART_IRQ, Rc::clone(&device_sifive.irq_pending));

    bus.borrow_mut().add_device(DeviceType {
        start: 0xc000_0000,
        len: 0x1000,
        instance: Box::new(device_sifive),
        name: "Sifive_Uart",
    });

    let hart0 = Rc::new(RefCell::new(
        CpuCoreBuild::new(bus, config)
            .with_boot_pc(0x8000_0000)
            .with_hart_id(0)
            .with_smode(true)
            .build(),
    ));

    let mut sim = RVsim::new(vec![hart0], 0);
    sim.load_image_from_slice(HELLO_BIN);
    sim.prepare_to_run();

    unsafe {
        SIM = Some(sim);
        UART_TX = Some(uart_tx_fifo);
        UART_RX = Some(uart_rx_fifo);
        CONSOLE_LINES = Vec::new();
        CUR_Y = 0;
        ANSI_PARSER = AnsiParser::new();
        CURRENT_COLOR = Color::WHITE;
        CURRENT_BOLD = false;
    }
}

#[gooseboy::update]
fn update(_nano_time: i64) {
    clear_framebuffer(Color::BLACK);

    unsafe {
        if let Some(rx_fifo) = UART_RX.as_mut() {
            while let Some(keycode) = gooseboy::input::get_key() {
                let bytes = keycode_to_bytes(keycode);
                for b in bytes {
                    rx_fifo.push(b);
                }
            }
        }
    }

    unsafe {
        let Some(sim) = SIM.as_mut() else { return };
        let Some(uart) = UART_TX.as_mut() else { return };

        sim.run_once(5_000 * 5);

        if CONSOLE_LINES.is_empty() {
            CONSOLE_LINES.push(Vec::new());
        }

        while let Some(b) = uart.pop() {
            if let Some(codes) = ANSI_PARSER.process_byte(b) {
                for code in codes {
                    match code {
                        AnsiCode::Reset => {
                            CURRENT_COLOR = Color::WHITE;
                            CURRENT_BOLD = false;
                        }
                        AnsiCode::Bold => {
                            CURRENT_BOLD = true;
                        }
                        AnsiCode::FgColor(n) => {
                            CURRENT_COLOR = ansi_to_color(n, CURRENT_BOLD);
                        }
                        AnsiCode::ClearScreen => {
                            CONSOLE_LINES.clear();
                            CONSOLE_LINES.push(Vec::new());
                            CUR_Y = 0;
                        }
                        AnsiCode::ClearLine => {
                            if let Some(line) = CONSOLE_LINES.last_mut() {
                                line.clear();
                            }
                        }
                        AnsiCode::Unknown => {}
                    }
                }

                continue;
            }

            let last_line = CONSOLE_LINES.last_mut();

            if b == b'\n' || b == b'\r' {
                if let Some(line) = last_line
                    && !line.is_empty()
                {
                    let s = line.iter().map(|c| c.ch).collect::<String>();
                    log!("{}", s);
                }
                CONSOLE_LINES.push(Vec::new());
                CUR_Y += 8;
            } else if let Some(line) = last_line {
                if b >= 32 || b == b'\t' {
                    let ch = b as char;
                    line.push(ConsoleCell {
                        ch,
                        color: CURRENT_COLOR,
                    });
                }
            } else {
                CONSOLE_LINES.push(vec![ConsoleCell {
                    ch: b as char,
                    color: CURRENT_COLOR,
                }]);
            }
        }

        let fb_height = get_framebuffer_height();
        let max_visual_lines = fb_height / 8;
        let fb_width = get_framebuffer_width();

        let mut total_visual_lines: usize = CONSOLE_LINES
            .iter()
            .map(|l| wrapped_line_count_cells(l, fb_width))
            .sum();

        while total_visual_lines > max_visual_lines && !CONSOLE_LINES.is_empty() {
            let removed = CONSOLE_LINES.remove(0);
            total_visual_lines =
                total_visual_lines.saturating_sub(wrapped_line_count_cells(&removed, fb_width));
        }

        let mut y_px = 0usize;
        let surface = gooseboy::framebuffer::get_framebuffer_surface_mut();
        for line in &CONSOLE_LINES {
            if y_px >= fb_height {
                break;
            }

            let mut x_px = 0usize;
            for cell in line {
                if x_px + 8 > fb_width {
                    y_px += 8;
                    x_px = 0;
                    if y_px >= fb_height {
                        break;
                    }
                }

                let s = &cell.ch.to_string();
                draw_text_wrapped_ex(
                    surface,
                    (x_px as i32).try_into().unwrap(),
                    (y_px as i32).try_into().unwrap(),
                    s,
                    cell.color,
                    None,
                );
                x_px += 8;
            }
            y_px += 8;
        }
    }
}

fn keycode_to_bytes(key: i32) -> Vec<u8> {
    match key {
        KEY_A..=KEY_Z => {
            let offset = (key - KEY_A) as u8;
            vec![b'a' + offset]
        }

        32..=126 => vec![key as u8],

        KEY_ENTER => vec![b'\r', b'\n'],
        KEY_TAB => vec![b'\t'],
        KEY_BACKSPACE => vec![0x08],
        KEY_ESCAPE => vec![0x1B],

        KEY_UP => vec![0x1B, b'[', b'A'],
        KEY_DOWN => vec![0x1B, b'[', b'B'],
        KEY_RIGHT => vec![0x1B, b'[', b'C'],
        KEY_LEFT => vec![0x1B, b'[', b'D'],

        KEY_HOME => vec![0x1B, b'[', b'H'],
        KEY_END => vec![0x1B, b'[', b'F'],
        KEY_PAGE_UP => vec![0x1B, b'[', b'5', b'~'],
        KEY_PAGE_DOWN => vec![0x1B, b'[', b'6', b'~'],
        KEY_INSERT => vec![0x1B, b'[', b'2', b'~'],
        KEY_DELETE => vec![0x1B, b'[', b'3', b'~'],

        _ => Vec::new(),
    }
}

fn wrapped_line_count_cells(line: &[ConsoleCell], max_width: usize) -> usize {
    if line.is_empty() {
        return 1;
    }
    let mut cx = 0usize;
    let mut lines = 1usize;
    for _cell in line {
        if cx + 8 > max_width {
            lines += 1;
            cx = 8;
        } else {
            cx += 8;
        }
    }
    lines
}

fn brighten(color: Color) -> Color {
    let factor = 1.5;

    Color {
        r: (f32::from(color.r) * factor).min(255.0) as u8,
        g: (f32::from(color.g) * factor).min(255.0) as u8,
        b: (f32::from(color.b) * factor).min(255.0) as u8,
        a: color.a,
    }
}

fn ansi_to_color(n: u8, bold: bool) -> Color {
    let base = match n {
        0 => Color::BLACK,
        1 => Color::RED,
        2 => Color::GREEN,
        3 => Color::YELLOW,
        4 => Color::BLUE,
        5 => Color::MAGENTA,
        6 => Color::CYAN,
        _ => Color::WHITE,
    };
    if bold { brighten(base) } else { base }
}
