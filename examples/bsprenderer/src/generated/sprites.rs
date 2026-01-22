// Auto-generated
use std::sync::LazyLock;
use gooseboy::sprite::Sprite;

#[allow(dead_code)]
pub static CAT: LazyLock<Sprite> = LazyLock::new(|| {
    let data = include_bytes!(concat!(env!("OUT_DIR"), "/cat.bin")); 
    Sprite::new_blended(256, 256, data)
});

