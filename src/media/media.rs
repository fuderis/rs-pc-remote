use crate::prelude::*;
use super::*;
use std::{
    process::{ Command, Stdio },
    os::windows::process::CommandExt
};

/// The audio devices controller
#[derive(Debug)]
pub struct Media {
    bin_path: Arc<PathBuf>,
    devices: Vec<Device>,
    active_audio: Option<Device>,
    active_micro: Option<Device>,
}

impl Media {
    /// Creates a new audio controller
    pub fn new<P: Into<PathBuf>>(bin_path: P) -> Result<Self> {
        let mut media = Self {
            bin_path: Arc::new(bin_path.into()),
            devices: vec![],
            active_audio: None,
            active_micro: None,
        };

        media.update_devices()?;

        Ok(media)
    }

    /// Updates the audio devices list
    pub fn update_devices(&mut self) -> Result<()> {
        // starting SVV for getting a devices list in CSV format:
        let output = Command::new(self.bin_path.join("svv/SoundVolumeView.exe"))
            .arg("/scomma")
            .creation_flags(0x08000000) // create no window
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        let mut devices = vec![];
        let mut active_audio = None;
        let mut active_micro = None;

        for line in stdout.lines().skip(1) {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 8 { continue; }

            // get name:
            let name = parts[0].trim_matches('"').trim().to_string();
            if name.is_empty() { continue }

            // get device kind:
            let device_type = parts[1].trim_matches('"');
            let device_type2 = parts[2].trim_matches('"');
            
            let kind = if &device_type[..] != "Device" {
                DeviceKind::Other
            } else {
                match &device_type2[..] {
                    "Render" => DeviceKind::Audio,
                    "Capture" => DeviceKind::Micro,
                    _ => DeviceKind::Other
                }
            };

            // check for active:
            let state = parts[7].trim_matches('"');
            let state_type = parts[5].trim_matches('"');
            let is_active = state == "Active" && (state_type == "Render" || state_type == "Capture");

            let device = Device {
                bin_path: self.bin_path.clone(),
                name,
                kind,
            };
            
            if is_active {
                match &device.kind {
                    DeviceKind::Audio => active_audio = Some(device.clone()),
                    DeviceKind::Micro => active_micro = Some(device.clone()),
                    _ => {}
                }
            }

            if !devices.contains(&device) {
                devices.push(device);
            }
        }

        if devices.is_empty() {
            return Err(Error::FoundNoDevices.into());
        }

        self.devices = devices;
        self.active_audio = active_audio;
        self.active_micro = active_micro;

        Ok(())
    }
    
    /// Returns the list of all audio devices
    pub fn devices(&self) -> &Vec<Device> {
        &self.devices
    }

    /// Returns the identifier of active audio device
    pub fn active_audio(&self) -> Option<&Device> {
        self.active_audio.as_ref()
    }

    /// Returns the identifier of active microphone device
    pub fn active_micro(&self) -> Option<&Device> {
        self.active_micro.as_ref()
    }
}
