use glam::{Mat3, Vec2};
use gooseboy::{
    color::Color,
    framebuffer::{
        Surface, clear_surface, get_framebuffer_height, get_framebuffer_ptr, get_framebuffer_width,
    },
    log, mem,
    sprite::{Sprite, blit_premultiplied_clipped},
    text::{draw_text_wrapped_ex, get_text_height, get_text_width},
};

use crate::transformer::{self, Resample};

type Transform = Mat3;

pub enum Command {
    Clear {
        color: Color,
    },
    Text {
        transform: Transform,
        text: String,
        color: Color,
        max_width: Option<usize>,
        resampling: Resample,
    },
    Sprite {
        transform: Transform,
        id: usize,
        color: Color,
        resampling: Resample,
    },
    Rect {
        transform: Transform,
        size: Vec2,
        color: Color,
        resampling: Resample,
    },
    BeginGroup {
        label: Option<String>,
    },
    EndGroup {},
}

pub struct AtlasEntry {
    id: usize,
    surface: Surface,
}

#[derive(Default)]
pub struct Renderer {
    commands: Vec<Command>,
    atlas: Vec<AtlasEntry>,
    next_atlas_id: usize,
    group_stack: Vec<String>,
}

impl Renderer {
    pub fn next_atlas_id(&mut self) -> usize {
        self.next_atlas_id += 1;
        self.next_atlas_id
    }

    pub fn upload_sprite(&mut self, sprite: &Sprite) -> usize {
        let id = self.next_atlas_id();
        self.atlas.push(AtlasEntry {
            id,
            surface: Surface::new(sprite.width, sprite.height, sprite.rgba.clone()),
        });
        id
    }

    pub fn group<F>(&mut self, label: &str, func: F)
    where
        F: FnOnce(&mut Self),
    {
        self.command(Command::BeginGroup {
            label: Some(label.to_owned()),
        });
        func(self);
        self.command(Command::EndGroup {});
    }

    pub fn clear(&mut self, color: Color) {
        self.command(Command::Clear { color });
    }

    pub fn command(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn flush(&mut self) {
        let result = self.process_commands();
        unsafe {
            mem::copy(
                get_framebuffer_ptr() as i32,
                result.rgba.as_ptr() as i32,
                result.rgba.len() as i32,
            )
        };
        if !self.group_stack.is_empty() {
            log!("axolotl: group stack is not empty, did you forget to EndGroup?");
            log!("axolotl: group list: {:?}", self.group_stack);
        }
    }
}

impl Renderer {
    pub fn process_command(&mut self, command: Command, surface: &mut Surface) {
        match command {
            Command::Clear { color } => {
                clear_surface(surface.rgba.as_ptr(), surface.rgba.len(), color);
            }
            Command::Text {
                transform,
                text,
                color,
                max_width,
                resampling,
            } => {
                let width = get_text_width(text.clone());
                let height = get_text_height(text.clone());
                let mut text_surface = Surface::new_empty(width, height);
                draw_text_wrapped_ex(&mut text_surface, 0, 0, text, color, max_width);
                let (out_width, out_height, off_x, off_y, transformed) =
                    transformer::transform_rgba(
                        &text_surface.rgba,
                        width,
                        height,
                        transform,
                        resampling,
                        true,
                    );
                blit_premultiplied_clipped(
                    surface,
                    off_x,
                    off_y,
                    out_width,
                    out_height,
                    &transformed,
                    true,
                );
            }
            Command::Sprite {
                transform,
                id,
                color,
                resampling,
            } => {
                let entry = self.atlas.iter().find(|p| p.id == id).unwrap();
                let (out_width, out_height, off_x, off_y, mut transformed) =
                    transformer::transform_rgba(
                        &entry.surface.rgba,
                        entry.surface.width,
                        entry.surface.height,
                        transform,
                        resampling,
                        true,
                    );
                transformer::tint_rgba(&mut transformed, color);
                blit_premultiplied_clipped(
                    surface,
                    off_x,
                    off_y,
                    out_width,
                    out_height,
                    &transformed,
                    true,
                );
            }
            Command::Rect {
                transform,
                size,
                color,
                resampling,
            } => {
                let rect_surface = Surface::new_empty(size.x as usize, size.y as usize);
                clear_surface(rect_surface.rgba.as_ptr(), rect_surface.rgba.len(), color);
                let (out_w, out_h, off_x, off_y, transformed) = transformer::transform_rgba(
                    &rect_surface.rgba,
                    rect_surface.width,
                    rect_surface.height,
                    transform,
                    resampling,
                    true,
                );
                blit_premultiplied_clipped(surface, off_x, off_y, out_w, out_h, &transformed, true);
            }
            Command::BeginGroup { label } => {
                if let Some(text) = label {
                    self.group_stack.push(text.to_string());
                }
            }
            Command::EndGroup {} => {
                self.group_stack.pop();
            }
        }
    }

    pub fn process_commands(&mut self) -> Surface {
        let mut surface = Surface::new_empty(get_framebuffer_width(), get_framebuffer_height());
        let commands = std::mem::take(&mut self.commands);

        for command in commands {
            self.process_command(command, &mut surface);
        }

        surface
    }
}
