// Auto-generated
use std::sync::LazyLock;
use gooseboy::sprite::Sprite;

#[allow(dead_code)]
pub static ICON: LazyLock<Sprite> = LazyLock::new(|| {
    let data = include_bytes!(concat!(env!("OUT_DIR"), "/icon.bin")); 
    Sprite::new_blended(128, 128, data)
});

