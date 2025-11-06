use glam::Mat3;
use gooseboy::{color::Color, sprite::Sprite};

type Transform = Mat3;

pub enum Command {
    Clear {
        color: Color,
    },
    Text {
        transform: Transform,
        text: String,
    },
    Sprite {
        transform: Transform,
        id: u8,
        color: Color,
    },
    Rect {
        transform: Transform,
        color: Color,
    },
}

pub struct AtlasEntry {
    id: usize,
    width: usize,
    height: usize,
    rgba: Vec<u8>,
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
            width: sprite.width,
            height: sprite.height,
            rgba: sprite.rgba,
        });
    }

    pub fn flush(&mut self) {
        // process commands here

        // flush
        self.commands.clear();
    }
}
