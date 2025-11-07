#![no_main]

use std::mem::MaybeUninit;

use gooseboy::framebuffer::init_fb;
use gooseboy::input::{
    get_mouse_x, get_mouse_y, is_mouse_button_down, is_mouse_button_just_pressed,
};
use gooseboy::log;
use gooseboy::text::draw_text;
use gooseboy::{color::Color, framebuffer::clear_framebuffer};
use libfnaf::Game;

use crate::animated_sprite::AnimatedSprite;

mod sprites {
    include!("generated/sprites.rs");
}

pub mod sounds {
    use gooseboy::{audio::Audio, make_audio};
    use std::sync::{LazyLock, Mutex};

    pub static FOXY_RUN: LazyLock<Mutex<Audio>> = make_audio!(foxy_run);
    pub static LIGHT: LazyLock<Mutex<Audio>> = make_audio!(error);
    pub static JUMPSCARE: LazyLock<Mutex<Audio>> = make_audio!(jumpscare_regular);
}

mod animated_sprite;

struct GameAnimatedSprites {
    idle_sprites: Vec<AnimatedSprite>,
    foxy_jumpscare: AnimatedSprite,
    left_door: AnimatedSprite,
    right_door: AnimatedSprite,
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

        use sprites::*;

        let fps = 30.0;

        ANIMATED_SPRITES = Some(GameAnimatedSprites {
            idle_sprites: vec![AnimatedSprite::new(
                vec![(*FAN_0).clone(), (*FAN_1).clone(), (*FAN_2).clone()],
                fps,
                159,
                88,
            )],
            foxy_jumpscare: AnimatedSprite::new(
                vec![
                    (*OFFICE_FOXY_0).clone(),
                    (*OFFICE_FOXY_1).clone(),
                    (*OFFICE_FOXY_2).clone(),
                    (*OFFICE_FOXY_3).clone(),
                    (*OFFICE_FOXY_4).clone(),
                    (*OFFICE_FOXY_5).clone(),
                    (*OFFICE_FOXY_6).clone(),
                    (*OFFICE_FOXY_7).clone(),
                    (*OFFICE_FOXY_8).clone(),
                    (*OFFICE_FOXY_9).clone(),
                    (*OFFICE_FOXY_10).clone(),
                    (*OFFICE_FOXY_11).clone(),
                    (*OFFICE_FOXY_12).clone(),
                    (*OFFICE_FOXY_13).clone(),
                    (*OFFICE_FOXY_14).clone(),
                    (*OFFICE_FOXY_15).clone(),
                    (*OFFICE_FOXY_16).clone(),
                ],
                fps,
                0,
                0,
            ),
            left_door: AnimatedSprite::new(vec![], fps, 0, 0),
            right_door: AnimatedSprite::new(vec![], fps, 0, 0),
        });
    }
}

static mut LAST_NANO_TIME: i64 = 0;
static mut LAST_TICK: i64 = 0;
const TICK_NS: i64 = 1_000_000_000 / libfnaf::FRAMES_PER_SECOND as i64;

fn play_sound(sound: &'static str) {
    match sound {
        "animatronic-camera-move.mp3" | "foxy-run.mp3" => {
            sounds::FOXY_RUN.lock().unwrap().play();
        }
        "error.mp3" => {
            sounds::LIGHT.lock().unwrap().play();
        }
        _ => {
            log!("unhandled sound: {}", sound);
        }
    }
}

#[allow(static_mut_refs)]
fn update_game(nano_time: i64) {
    unsafe {
        if LAST_TICK == 0 {
            LAST_TICK = nano_time;
        }

        let elapsed = nano_time - LAST_TICK;

        if elapsed >= TICK_NS {
            let ticks = (elapsed / TICK_NS) * 20;

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
                            play_sound(sound);
                        }
                        libfnaf::GameEvent::Jumpscare { by } => {
                            log!("jumpscare by {:?}", by);
                            sounds::JUMPSCARE.lock().unwrap().play();
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

fn get_power_usage(game: &Game) -> u32 {
    let mut count = 0u32;
    if game.left_door_closed {
        count += 1;
    }
    if game.right_door_closed {
        count += 1;
    }
    if game.cameras_on {
        count += 1;
    }
    if game.left_light_on {
        count += 1;
    }
    if game.right_light_on {
        count += 1;
    }

    core::cmp::min(count, 4)
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

    let animated_sprites = unsafe { &mut ANIMATED_SPRITES.as_mut().unwrap() };

    let game = unsafe { FNAF.as_mut().unwrap() };

    if game.left_light_on && game.right_light_on {
        sprites::OFFICE_NORMAL.blit(0, 0);
    } else if game.left_light_on {
        sprites::OFFICE_LIGHT_LEFT.blit(0, 0);
    } else if game.right_light_on {
        sprites::OFFICE_LIGHT_RIGHT.blit(0, 0);
    } else if game.power_out {
        sprites::OFFICE_POWER_OUT.blit(0, 0);
    } else if game.bonnie.in_office {
        sprites::OFFICE_BONNIE.blit(0, 0);
    } else if game.chica.in_office {
        sprites::OFFICE_CHICA.blit(0, 0);
    } else if game.freddy.in_office {
        sprites::JUMPSCARE_FREDDY_0.blit(0, 0);
    } else if game.foxy.in_office {
        animated_sprites.foxy_jumpscare.update(dt);
    } else {
        sprites::OFFICE_NORMAL.blit(0, 0);
    }

    let left_trigger_sprite = match (game.left_door_closed, game.left_light_on) {
        (true, true) => &sprites::TRIGGER_LEFT_DOOR_LIGHT,
        (true, false) => &sprites::TRIGGER_LEFT_DOOR,
        (false, true) => &sprites::TRIGGER_LEFT_LIGHT,
        (false, false) => &sprites::TRIGGER_LEFT_NONE,
    };
    left_trigger_sprite.blit(0, 50);

    let right_trigger_sprite = match (game.right_door_closed, game.right_light_on) {
        (true, true) => &sprites::TRIGGER_RIGHT_DOOR_LIGHT,
        (true, false) => &sprites::TRIGGER_RIGHT_DOOR,
        (false, true) => &sprites::TRIGGER_RIGHT_LIGHT,
        (false, false) => &sprites::TRIGGER_RIGHT_NONE,
    };
    right_trigger_sprite.blit(290, 50);

    if is_mouse_button_just_pressed(0) {
        let mx = get_mouse_x();
        let my = get_mouse_y();

        let x = 0;
        let y = 50;
        let w = 35;
        let h = 95;
        if (x..=x + w).contains(&mx) && (y..=y + (h / 2)).contains(&my) {
            game.toggle_door_left();
        } else if (x..=x + w).contains(&mx) && (y + (h / 2)..=y + h).contains(&my) {
            game.toggle_light_left();
        }

        let x = 290;
        let y = 50;
        if (x..=x + w).contains(&mx) && (y..=y + (h / 2)).contains(&my) {
            game.toggle_door_right();
        } else if (x..=x + w).contains(&mx) && (y + (h / 2)..=y + h).contains(&my) {
            game.toggle_light_right();
        }
    }

    let power_usage = get_power_usage(game);
    let power_sprite = match power_usage {
        0 => &sprites::POWER_1,
        1 => &sprites::POWER_1,
        2 => &sprites::POWER_2,
        3 => &sprites::POWER_3,
        4 => &sprites::POWER_4,
        _ => &sprites::POWER_5,
    };

    power_sprite.blit(0, 180);

    for ele in &mut animated_sprites.idle_sprites {
        ele.update(dt);
    }

    draw_text(
        0,
        0,
        format!(
            "Time: {} Power: {}",
            game.time_of_night_seconds,
            game.power_percent.round()
        ),
        Color::RED,
    );
}
