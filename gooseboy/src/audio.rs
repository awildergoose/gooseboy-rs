use crate::bindings::{play_audio, stop_audio};

pub struct Audio {
    data: Vec<u8>,
}

impl Audio {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn play(&mut self) -> Option<AudioInstance> {
        let id = unsafe { play_audio(self.data.as_ptr() as i32, self.data.len() as i32) };
        if id == -1 {
            return None;
        }
        Some(AudioInstance::new(id))
    }
}

pub struct AudioInstance {
    id: i64,
}

impl AudioInstance {
    fn new(id: i64) -> Self {
        Self { id }
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
