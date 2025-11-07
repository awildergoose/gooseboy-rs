use gooseboy::sprite::Sprite;

pub struct AnimatedSprite {
    sprites: Vec<Sprite>,
    fps: f32,
    pub frame: u32,
    frame_timer: f32,
    pub x: usize,
    pub y: usize,
}

impl AnimatedSprite {
    pub fn new(sprites: Vec<Sprite>, fps: f32, x: usize, y: usize) -> Self {
        Self {
            sprites,
            fps,
            x,
            y,
            frame: 0,
            frame_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.frame_timer += dt;
        if self.frame_timer >= 1.0 / self.fps {
            self.advance_frame();
        }

        let sprite = &self.sprites[self.frame as usize];
        sprite.blit(self.x, self.y);
    }

    pub fn advance_frame(&mut self) {
        self.frame = (self.frame + 1) % self.sprites.len() as u32;
        self.frame_timer = 0.0;
    }
}
