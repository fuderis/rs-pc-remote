use crate::prelude::*;
use std::{
    process::Command,
    os::windows::process::CommandExt
};

/// The type of media device
#[derive(Hash, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceKind {
    Audio,
    Micro,
    Other,
}

/// The media device
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub(super) bin_path: Arc<PathBuf>,
    pub name: String,
    pub kind: DeviceKind,
}

impl Device {
    /// Checks for audio type
    pub fn is_audio(&self) -> bool {
        &self.kind == &DeviceKind::Audio
    }

    /// Checks for microphone type
    pub fn is_micro(&self) -> bool {
        &self.kind == &DeviceKind::Micro
    }

    /// Checks for other type
    pub fn is_other(&self) -> bool {
        &self.kind == &DeviceKind::Other
    }
    
    /*/// Checks device for active
    pub fn is_active(&self) -> bool {
        todo!()
    }*/

    /// Sets the current audio device as active
    pub fn set_as_active(&self) -> Result<()> {
        let status = Command::new(self.bin_path.join("svv/SoundVolumeView.exe"))
            .arg("/SetDefault")
            .arg(&self.name)
            .arg(match self.kind {
                DeviceKind::Other => return Err(Error::ExpectedAudioDevice(self.name.clone()).into()),
                _ => "all",
            })
            .creation_flags(0x08000000)
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(Error::FailedSetDefaultDevice(self.name.clone()).into())
        }
    }

    /// Returns the audio volume
    pub fn get_volume(&self) -> Result<u32> {
        let status = Command::new(&self.bin_path.join("svcl/svcl.exe"))
            .arg("/GetPercent")
            .arg(&self.name)
            .creation_flags(0x08000000)
            .status()?;

        let code = status.code().unwrap_or(0) as u32;
        let volume = code / 10;

        Ok(volume)
    }

    /// Changes the audio volume
    pub fn set_volume(&self, volume: u32) -> Result<()> {
        let sys_volume = (volume * 65535) / 100;
        
        let status = Command::new(&self.bin_path.join("nircmd/nircmd.exe"))
            .arg("setsysvolume")
            .arg(sys_volume.to_string())
            .creation_flags(0x08000000)
            .status()?;
        
        if !status.success() { return Err(Error::FailedSetVolume(self.name.clone()).into()); }
        
        Ok(())
    }

    /// Changes the audio volume (relative)
    pub fn set_volume_relative(&self, delta: i32) -> Result<()> {
        let new_volume = (self.get_volume()? as i32 + delta).clamp(0, 100);
        self.set_volume(new_volume as u32)
    }

    /// Checks the device for muted volume
    pub fn is_muted(&self) -> Result<bool> {
        let status = Command::new(&self.bin_path.join("svcl/svcl.exe"))
            .arg("/GetMute")
            .arg(&self.name)
            .creation_flags(0x08000000)
            .status()?; // 1 = muted, 0 = unmuted
        
        Ok(status.code().map(|code| code == 1).unwrap_or(false))
    }
    
    /// Mutes/unmutes the device
    pub fn mute_unmute(&self) -> Result<()> {
        let status = Command::new(self.bin_path.join("svv/SoundVolumeView.exe"))
            .arg("/Switch")
            .arg(&self.name)
            .creation_flags(0x08000000)
            .status()?;
        
        if status.success() {
            Ok(())
        } else {
            Err(Error::FailedSwitchDeviceMute(self.name.clone()).into())
        }
    }

    /// Mutes the device
    pub fn mute(&self) -> Result<()> {
        if !self.is_muted()? { self.mute_unmute() }
        else { Ok(()) }
    }

    /// Unmutes the device
    pub fn unmute(&self) -> Result<()> {
        if self.is_muted()? { self.mute_unmute() }
        else { Ok(()) }
    }
}

impl PartialOrd for DeviceKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DeviceKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let order = |d: &DeviceKind| match d {
            DeviceKind::Audio => 0,
            DeviceKind::Micro => 1,
            DeviceKind::Other => 2,
        };
        order(self).cmp(&order(other))
    }
}
