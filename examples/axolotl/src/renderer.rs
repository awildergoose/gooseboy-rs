use glam::{Mat3, Vec2};
use gooseboy::{
    color::Color,
    framebuffer::{
        Surface, clear_surface, get_framebuffer_height, get_framebuffer_ptr, get_framebuffer_width,
    },
    mem,
    sprite::{Sprite, blit_ex},
    text::{draw_text_wrapped_ex, get_text_height, get_text_width},
};

use crate::transformer;

type Transform = Mat3;

pub enum Command {
    Clear {
        color: Color,
    },
    Text {
        transform: Transform,
        text: String,
        color: Color,
    },
    Sprite {
        transform: Transform,
        id: usize,
        color: Color,
    },
    Rect {
        transform: Transform,
        size: Vec2,
        color: Color,
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
    // TODO cache transforms here
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
        self.commands.clear();
    }
}

impl Renderer {
    pub fn process_command(&self, command: &Command, surface: &mut Surface) {
        match command {
            Command::Clear { color } => {
                clear_surface(surface.rgba.as_ptr(), surface.rgba.len(), *color);
            }
            Command::Text {
                transform,
                text,
                color,
            } => {
                let width = get_text_width(text);
                let height = get_text_height(text);
                let mut text_surface = Surface::new_empty(width, height);
                draw_text_wrapped_ex(&mut text_surface, 0, 0, text, *color, None);
                let (out_width, out_height, transformed) = transformer::transform_rgba(
                    &text_surface.rgba,
                    width,
                    height,
                    *transform,
                    transformer::Resample::Nearest,
                    true,
                );
                blit_ex(surface, 0, 0, out_width, out_height, &transformed, true);
            }
            Command::Sprite {
                transform,
                id,
                color,
            } => {
                let entry = self.atlas.iter().find(|p| p.id == *id).unwrap();
                let (out_width, out_height, mut transformed) = transformer::transform_rgba(
                    &entry.surface.rgba,
                    entry.surface.width,
                    entry.surface.height,
                    *transform,
                    transformer::Resample::Bilinear,
                    true,
                );
                transformer::tint_rgba(&mut transformed, *color);
                // TODO self.cached_transforms
                blit_ex(surface, 0, 0, out_width, out_height, &transformed, true);
            }
            Command::Rect {
                transform,
                size,
                color,
            } => {
                let rect_surface = Surface::new_empty(size.x as usize, size.y as usize);
                clear_surface(rect_surface.rgba.as_ptr(), rect_surface.rgba.len(), *color);
                let (out_width, out_height, transformed) = transformer::transform_rgba(
                    &rect_surface.rgba,
                    rect_surface.width,
                    rect_surface.height,
                    *transform,
                    transformer::Resample::Nearest,
                    true,
                );
                blit_ex(surface, 0, 0, out_width, out_height, &transformed, true);
            }
            Command::BeginGroup { label } => todo!(),
            Command::EndGroup {} => todo!(),
        }
    }

    pub fn process_commands(&self) -> Surface {
        let mut surface = Surface::new_empty(get_framebuffer_width(), get_framebuffer_height());

        for command in &self.commands {
            self.process_command(command, &mut surface);
        }

        surface
    }
}
