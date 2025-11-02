use crate::bindings::play_audio;

#[macro_export]
macro_rules! import_audio {
    ($name:ident) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/raw/", stringify!($name), ".raw"))
    };
}

#[macro_export]
macro_rules! make_audio {
    ($name:ident) => {
        std::sync::LazyLock::new(|| Audio::new(import_audio!($name).to_vec()))
    };
}

pub struct Audio {
    data: Vec<u8>,
}

impl Audio {
    pub fn new(data: Vec<u8>) -> Audio {
        Audio { data }
    }

    pub fn play(&self) {
        unsafe { play_audio(self.data.as_ptr() as i32, self.data.len() as i32) };
    }
}
