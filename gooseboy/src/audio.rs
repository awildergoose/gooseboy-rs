use crate::bindings::{
    self, is_audio_playing, play_audio, set_audio_pitch, set_audio_volume, stop_audio,
};

/// Requires AUDIO permission
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
    volume: f32,
    pitch: f32,
}

impl AudioInstance {
    fn new(id: i64) -> Self {
        Self {
            id,
            volume: 1.0,
            pitch: 1.0,
        }
    }

    pub fn stop(&mut self) {
        unsafe {
            stop_audio(self.id);
        }
        self.id = -1;
    }

    pub fn set_volume(&mut self, new: f32) {
        unsafe {
            set_audio_volume(self.id, new);
        }
        self.volume = new;
    }

    pub fn set_pitch(&mut self, new: f32) {
        unsafe {
            set_audio_pitch(self.id, new);
        }
        self.pitch = new;
    }

    pub fn is_playing(&self) -> bool {
        unsafe { is_audio_playing(self.id) }
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn get_pitch(&self) -> f32 {
        self.pitch
    }
}

#[macro_export]
macro_rules! import_audio {
    ($name:ident) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/raw/", stringify!($name), ".raw"))
    };
}

/// Requires AUDIO permission
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

pub fn stop_all_audio() {
    unsafe {
        bindings::stop_all_audio();
    }
}
