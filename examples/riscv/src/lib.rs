#![allow(static_mut_refs)]
#![no_main]

use std::cell::RefCell;
use std::rc::Rc;

use gooseboy::{
    color::Color,
    framebuffer::{get_framebuffer_width, init_fb},
    text::draw_text_wrapped_ex,
};
use rv64emu::{
    config::Config,
    device::{
        device_am_uart::DeviceUart,
        device_memory::DeviceMemory,
        device_trait::{MEM_BASE, SERIAL_PORT},
    },
    rv64core::{
        bus::{Bus, DeviceType},
        cpu_core::CpuCoreBuild,
    },
    rvsim::RVsim,
    tools::{FifoUnbounded, fifo_unbounded_new, rc_refcell_new},
};

const HELLO_BIN: &[u8] = include_bytes!("../hello.bin");

static mut SIM: Option<RVsim> = None;
static mut UART_TX: Option<FifoUnbounded<u8>> = None;
static mut CONSOLE_LINES: Vec<String> = Vec::new();
static mut CUR_Y: usize = 0;

#[gooseboy::main]
fn main() {
    init_fb();

    let mut config = Config::new();
    config.set_mmu_type("bare");
    config.set_isa("rv64im");
    let config = Rc::new(config);

    let bus = rc_refcell_new(Bus::new());
    let hart0 = Rc::new(RefCell::new(
        CpuCoreBuild::new(bus.clone(), config.clone())
            .with_boot_pc(0x8000_0000)
            .with_hart_id(0)
            .with_smode(false)
            .build(),
    ));

    let mem = DeviceMemory::new(8 * 1024 * 1024);
    bus.borrow_mut().add_device(DeviceType {
        start: MEM_BASE,
        len: mem.size() as u64,
        instance: Box::new(mem),
        name: "RAM",
    });

    let uart_tx_fifo = fifo_unbounded_new::<u8>();
    let uart = DeviceUart::new(uart_tx_fifo.clone());
    bus.borrow_mut().add_device(DeviceType {
        start: SERIAL_PORT,
        len: 1,
        instance: Box::new(uart),
        name: "UART",
    });

    let mut sim = RVsim::new(vec![hart0.clone()], 0);
    sim.load_image_from_slice(HELLO_BIN);
    sim.prepare_to_run();

    unsafe {
        SIM = Some(sim);
        UART_TX = Some(uart_tx_fifo);
        CONSOLE_LINES = Vec::new();
        CUR_Y = 0;
    }
}

#[gooseboy::update]
fn update(_nano_time: i64) {
    unsafe {
        let sim = match SIM.as_mut() {
            Some(s) => s,
            None => return,
        };
        let uart = match UART_TX.as_mut() {
            Some(u) => u,
            None => return,
        };

        sim.run_once(5000);

        while let Some(b) = uart.pop() {
            let last_line = CONSOLE_LINES.last_mut();
            if b == b'\n' {
                CONSOLE_LINES.push(String::new());
                CUR_Y += 8;
            } else if let Some(line) = last_line {
                line.push(b as char);
            } else {
                CONSOLE_LINES.push((b as char).to_string());
            }
        }

        let fb_height = 200;
        let max_lines = fb_height / 8;
        if CONSOLE_LINES.len() > max_lines {
            let excess = CONSOLE_LINES.len() - max_lines;
            CONSOLE_LINES.drain(0..excess);
            CUR_Y = max_lines * 8 - 8;
        }

        for (line_index, line) in CONSOLE_LINES.iter().enumerate() {
            draw_text_wrapped_ex(
                gooseboy::framebuffer::get_framebuffer_surface_mut(),
                0,
                line_index * 8,
                line,
                Color::WHITE,
                Some(get_framebuffer_width()),
            );
        }
    }
}
