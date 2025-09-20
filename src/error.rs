use macron::{ Display, From, Error };

/// Std Result alias
pub type StdResult<T, E> = std::result::Result<T, E>;
/// Result alias
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

// The error
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[from]
    Io(std::io::Error),

    #[display = "Failed to get the audio devices list"]
    FoundNoDevices,

    #[display = "Expected an audio device or microphone, found: '{0}'"]
    ExpectedAudioDevice(String),

    #[display = "Failed to set active device: '{0}'"]
    FailedSetDefaultDevice(String),

    #[display = "Failed to set volume to device: '{0}'"]
    FailedSetVolume(String),

    #[display = "Failed to switch device mute: '{0}'"]
    FailedSwitchDeviceMute(String),
}
