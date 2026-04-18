use crate::{
    bindings::{self, is_audio_playing, play_audio, set_audio_pitch, set_audio_volume, stop_audio},
    unsafe_casts,
};

#[repr(i32)]
pub enum AudioFormat {
    Mono8,
    Mono16,
    Stereo8,
    Stereo16,
}

impl AudioFormat {
    #[must_use]
    pub const fn repr(&self) -> i32 {
        match self {
            Self::Mono8 => 0x1100,
            Self::Mono16 => 0x1101,
            Self::Stereo8 => 0x1102,
            Self::Stereo16 => 0x1103,
        }
    }
}

/// Requires Audio permission
pub struct Audio {
    data: Vec<u8>,
    sample_rate: i32,
    format: AudioFormat,
}

impl Audio {
    #[must_use]
    pub const fn new(data: Vec<u8>, sample_rate: i32, format: AudioFormat) -> Self {
        Self {
            data,
            sample_rate,
            format,
        }
    }

    pub fn play(&mut self) -> Option<AudioInstance> {
        let id = unsafe {
            play_audio(
                self.data.as_ptr(),
                unsafe_casts::arr_len(&self.data),
                self.sample_rate,
                self.format.repr(),
            )
        };
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
    #[must_use]
    pub const fn new(id: i64) -> Self {
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

    #[must_use]
    pub fn is_playing(&self) -> bool {
        unsafe { is_audio_playing(self.id) }
    }

    #[must_use]
    pub const fn get_volume(&self) -> f32 {
        self.volume
    }

    #[must_use]
    pub const fn get_pitch(&self) -> f32 {
        self.pitch
    }
}

#[macro_export]
macro_rules! import_audio {
    ($name:ident) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/raw/", stringify!($name), ".raw"))
    };
}

/// Requires Audio permission
#[macro_export]
macro_rules! make_audio {
    ($name:ident, $sample_rate:expr, $format:ident) => {
        std::sync::LazyLock::new(|| {
            std::sync::Mutex::new($crate::audio::Audio::new(
                $crate::import_audio!($name).to_vec(),
                $sample_rate,
                $crate::audio::AudioFormat::$format,
            ))
        })
    };
}

pub fn stop_all_audio() {
    unsafe {
        bindings::stop_all_audio();
    }
}
