use glam::{Mat3, Vec2};
use gooseboy::{
    color::Color,
    framebuffer::{Surface, clear_surface, get_framebuffer_height, get_framebuffer_width},
    sprite::{Sprite, blit_ex},
    text::draw_text,
};

use crate::transformer;

type Transform = Mat3;

pub enum Command {
    Clear {
        color: Color,
    },
    Text {
        transform: Transform,
        position: Vec2,
        text: String,
        color: Color,
    },
    Sprite {
        transform: Transform,
        position: Vec2,
        id: usize,
        color: Color,
    },
    Rect {
        transform: Transform,
        position: Vec2,
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

    pub fn upload_sprite(&mut self, sprite: Sprite) {
        let id = self.next_atlas_id();
        self.atlas.push(AtlasEntry {
            id,
            surface: Surface::new(sprite.width, sprite.height, sprite.rgba),
        });
    }

    pub fn flush(&mut self) {
        self.process_commands();
        self.commands.clear();
    }
}

impl Renderer {
    pub fn process_commands(&self) -> Surface {
        let surface = Surface::new_empty(get_framebuffer_width(), get_framebuffer_height());

        for command in &self.commands {
            match command {
                Command::Clear { color } => {
                    clear_surface(surface.rgba.as_ptr(), surface.rgba.len(), *color);
                }
                Command::Text {
                    transform,
                    position,
                    text,
                    color,
                } => {
                    // TODO apply transforms
                    draw_text(position.x as usize, position.y as usize, text, *color);
                }
                Command::Sprite {
                    transform,
                    position,
                    id,
                    color,
                } => {
                    let entry = self.atlas.iter().find(|p| p.id == *id).unwrap();
                    let (out_width, out_height, mut transformed) = transformer::transform_rgba(
                        &entry.surface.rgba,
                        entry.surface.width,
                        entry.surface.height,
                        *transform,
                    );
                    transformer::tint_rgba(&mut transformed, *color);
                    // TODO self.cached_transforms
                    blit_ex(
                        position.x as usize,
                        position.y as usize,
                        out_width,
                        out_height,
                        &transformed,
                        true,
                    );
                }
                Command::Rect {
                    transform,
                    position,
                    color,
                } => todo!(),
                Command::BeginGroup { label } => todo!(),
                Command::EndGroup {} => todo!(),
            }
        }

        surface
    }
}
