use crate::bindings::{play_audio, stop_audio};

pub struct Audio {
    data: Vec<u8>,
    id: i64,
}

impl Audio {
    pub fn new(data: Vec<u8>) -> Audio {
        Audio { data, id: -1 }
    }

    pub fn play(&mut self) {
        self.id = unsafe { play_audio(self.data.as_ptr() as i32, self.data.len() as i32) };
    }

    pub fn stop(&mut self) {
        unsafe {
            stop_audio(self.id);
        }
        self.id = -1;
    }
}

#[macro_export]
macro_rules! import_audio {
    ($name:ident) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/raw/", stringify!($name), ".raw"))
    };
}

#[macro_export]
macro_rules! make_audio {
    ($name:ident) => {
        std::sync::LazyLock::new(|| {
            std::sync::Mutex::new($crate::audio::Audio::new(
                $crate::import_audio!($name).to_vec(),
            ))
        })
    };
}
