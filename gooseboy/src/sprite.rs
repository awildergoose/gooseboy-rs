use crate::framebuffer::get_framebuffer_surface_mut;

/// A sprite.
#[derive(Clone, Debug)]
pub struct Sprite {
    /// The width.
    pub width: usize,
    /// The height.
    pub height: usize,
    /// The RGBA values.
    pub rgba: Vec<u8>,
    /// Should this sprite blend with the background?
    pub blend: bool,
}

impl Sprite {
    /// Creates a new [`Sprite`].
    #[must_use]
    pub fn new(width: usize, height: usize, rgba: &[u8]) -> Self {
        Self {
            width,
            height,
            rgba: rgba.to_vec(),
            blend: false,
        }
    }

    /// Creates a new blended [`Sprite`].
    #[must_use]
    pub fn new_blended(width: usize, height: usize, rgba: &[u8]) -> Self {
        Self {
            width,
            height,
            rgba: rgba.to_vec(),
            blend: true,
        }
    }

    /// Blits the sprite onto a position.
    pub fn blit(&self, x: usize, y: usize) {
        get_framebuffer_surface_mut().blit_premultiplied_clipped(
            unsafe { crate::unsafe_casts::usize_as_i32(x) },
            unsafe { crate::unsafe_casts::usize_as_i32(y) },
            self.width,
            self.height,
            &self.rgba,
            self.blend,
        );
    }
}
