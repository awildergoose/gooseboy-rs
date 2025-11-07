#![no_main]

use std::mem::MaybeUninit;
use std::sync::{LazyLock, Mutex};

use axolotl::renderer::Renderer;
use gooseboy::framebuffer::init_fb;
use gooseboy::input::{get_mouse_x, get_mouse_y};
use gooseboy::log;
use gooseboy::sprite::Sprite;
use gooseboy::text::draw_text;
use gooseboy::{color::Color, framebuffer::clear_framebuffer};
use libfnaf::Game;

use crate::animated_sprite::AnimatedSprite;

mod sprites {
    include!("generated/sprites.rs");
}

mod animated_sprite;

struct GameAnimatedSprites {
    fan: AnimatedSprite,
}

static mut FNAF: Option<Game> = None;
static mut ANIMATED_SPRITES: Option<GameAnimatedSprites> = None;
// static RENDERER: Mutex<LazyLock<Renderer>> =
//     Mutex::new(std::sync::LazyLock::new(Renderer::default));

#[gooseboy::main]
fn main() {
    init_fb();

    unsafe {
        FNAF = Some(Game::new(1, 1));

        let frames = vec![
            (*sprites::FAN_0).clone(),
            (*sprites::FAN_1).clone(),
            (*sprites::FAN_2).clone(),
        ];

        ANIMATED_SPRITES = Some(GameAnimatedSprites {
            fan: AnimatedSprite::new(frames, 30.0, 159, 88),
        });
    }
}

static mut LAST_NANO_TIME: i64 = 0;
static mut LAST_TICK: i64 = 0;
const TICK_NS: i64 = 1_000_000_000 / libfnaf::FRAMES_PER_SECOND as i64;

#[allow(static_mut_refs)]
fn update_game(nano_time: i64) {
    unsafe {
        if LAST_TICK == 0 {
            LAST_TICK = nano_time;
        }

        let elapsed = nano_time - LAST_TICK;

        if elapsed >= TICK_NS {
            let ticks = elapsed / TICK_NS;

            for _ in 0..ticks {
                let fnaf = FNAF.as_mut().unwrap();
                fnaf.tick_frame();

                let mut events_buf: [MaybeUninit<libfnaf::GameEvent>; 16] =
                    MaybeUninit::uninit().assume_init();
                let count = fnaf.drain_events(&mut events_buf);

                for ev in events_buf.iter().take(count) {
                    let event = ev.assume_init_read();
                    match event {
                        libfnaf::GameEvent::Sound(sound) => {
                            log!("play sound: {}", sound);
                        }
                        unhandled => {
                            log!("unhandled event: {:?}", unhandled);
                        }
                    }
                }
            }

            LAST_TICK += ticks * TICK_NS;
        }
    }
}

#[allow(static_mut_refs)]
#[gooseboy::update]
fn update(nano_time: i64) {
    let dt = (nano_time - unsafe { LAST_NANO_TIME }) as f32 / 1_000_000_000.0;
    unsafe {
        LAST_NANO_TIME = nano_time;
    }

    clear_framebuffer(Color::BLACK);

    update_game(nano_time);

    // let mut binding = RENDERER.lock();
    // let r = binding.as_mut().unwrap();
    // r.clear(Color::BLACK);

    // r.flush();

    let fnaf = unsafe { FNAF.as_mut().unwrap() };

    if fnaf.left_light_on {
        sprites::OFFICE_LIGHT_LEFT.blit(0, 0);
    } else if fnaf.right_light_on {
        sprites::OFFICE_LIGHT_RIGHT.blit(0, 0);
    } else {
        sprites::OFFICE_NORMAL.blit(0, 0);
    }

    unsafe {
        let fan = &mut ANIMATED_SPRITES.as_mut().unwrap().fan;
        fan.update(dt);
        // fan.x = get_mouse_x() as usize;
        // fan.y = get_mouse_y() as usize + 20;
    }

    draw_text(
        0,
        0,
        format!("{} {}", get_mouse_x(), get_mouse_y()),
        Color::RED,
    );
}
