use crate::{
    bindings::{self, is_audio_playing, play_audio, set_audio_pitch, set_audio_volume, stop_audio},
    unsafe_casts,
};

#[repr(i32)]
/// The playback format of the audio, with [Stereo16] being the default.
pub enum AudioFormat {
    /// Mono 8.
    Mono8,
    /// Mono 16.
    Mono16,
    /// Stereo 8.
    Stereo8,
    /// Stereo 16, Also the default.
    Stereo16,
}

impl AudioFormat {
    /// Returns the OpenAL (backend) representation of the audio format.
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
    /// Creates a new `Audio`, to be played once or multiple times.
    /// You should always create only one instance of an `Audio` for an audio file,
    /// and when wanting to play, call [play](Audio::play)
    #[must_use]
    pub const fn new(data: Vec<u8>, sample_rate: i32, format: AudioFormat) -> Self {
        Self {
            data,
            sample_rate,
            format,
        }
    }

    /// Plays the `Audio` and returns an `AudioInstance` if successful.
    /// Fails if there are too many sounds currently playing.
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

/// An instance of an `Audio`.
pub struct AudioInstance {
    id: i64,
    volume: f32,
    pitch: f32,
}

impl AudioInstance {
    /// Creates a new `AudioInstance` from an `id`.
    /// You shouldn't use this unless you're manually using unsafe bindings.
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self {
            id,
            volume: 1.0,
            pitch: 1.0,
        }
    }

    /// Stops the `AudioInstance`, and therefore you should drop this `AudioInstance`
    /// as the ID is now invalid.
    pub fn stop(&mut self) {
        unsafe {
            stop_audio(self.id);
        }
        self.id = -1;
    }

    /// Sets the volume of the `AudioInstance`, ranging from 0 to 10.
    pub fn set_volume(&mut self, new: f32) {
        unsafe {
            set_audio_volume(self.id, new);
        }
        self.volume = new;
    }

    /// Sets the pitch of the `AudioInstance`, ranging from 0.1 to 10.
    pub fn set_pitch(&mut self, new: f32) {
        unsafe {
            set_audio_pitch(self.id, new);
        }
        self.pitch = new;
    }

    /// Is this `AudioInstance` currently playing?
    #[must_use]
    pub fn is_playing(&self) -> bool {
        unsafe { is_audio_playing(self.id) }
    }

    /// Returns the volume of this `AudioInstance`.
    /// Note that the returned value is not fetched from the host,
    /// but rather it's the stored value in this struct.
    #[must_use]
    pub const fn get_volume(&self) -> f32 {
        self.volume
    }

    /// Returns the pitch of this `AudioInstance`.
    /// Note that the returned value is not fetched from the host,
    /// but rather it's the stored value in this struct.
    #[must_use]
    pub const fn get_pitch(&self) -> f32 {
        self.pitch
    }
}

/// `include_bytes` an audio file from `OUT_DIR/raw/$name.raw`
#[macro_export]
macro_rules! import_audio {
    ($name:ident) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/raw/", stringify!($name), ".raw"))
    };
}

/// Creates a new `Audio` file, file is grabbed from the `OUT_DIR` that
/// the buildscript emits to.
/// Returns a `LazyLock<Mutex<Audio>>`
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

/// Stops all running `AudioInstance`s
pub fn stop_all_audio() {
    unsafe {
        bindings::stop_all_audio();
    }
}
