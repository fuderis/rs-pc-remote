[![github]](https://github.com/fuderis/rs-pc-remote)&ensp;
[![crates-io]](https://crates.io/crates/pc-remote)&ensp;
[![docs-rs]](https://docs.rs/pc-remote)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# PC Remote Control Library

This library provides cross-platform control over computer input devices. It allows emulating keyboard and mouse inputs programmatically and managing audio devices.
<br>
The library is designed for seamless integration into Rust projects, providing ergonomic APIs for all supported functionalities.
Please ensure the utility binaries (nircmd.exe, SoundVolumeView.exe, svcl.exe) are present in your binary path for audio control.


## Audio Device Management (for Windows OS only):

For audio device management, this library relies on several lightweight Windows utilities:

* **nircmd** — for setting system volume.
* **SoundVolumeView (svv)** — for managing default audio devices and mute/unmute operations.
* **SoundVolumeCommandLine (svcl)** — for retrieving current volume and mute status.

These tools are efficient and minimal, allowing fast interactions with **Windows** audio system without complex API integrations.
You can download these utilities bundled together here: https://github.com/fuderis/rs-pc-remote/raw/main/bin


## Examples:

### Keyboard Device Emulation:
```rust
use pc_remote::{ Result, Keyboard, Key };

fn main() -> Result<()> {
    let mut keyboard = Keyboard::new()?;

    let hotkey = [Key::Shift, Key::A];
    
    keyboard.press(&hotkey)?;
    keyboard.release(&hotkey)?;

    Ok(())
}
```

### Mouse Device Emulation:
```rust
use pc_remote::{ Result, Mouse };

fn main() -> Result<()> {
    let mut mouse = Mouse::new()?;

    mouse.move_to_center()?;
    
    Ok(())
}
```

### Media Devices Control:
```rust
use pc_remote::{ Result, Media };
use macron::path;

fn main() -> Result<()> {
    let media = Media::new(path!("bin/nirsoft"))?;

    /* binary repository example:
        /bin
            /nirsoft
                /nircmd/nircmd.exe
                /svcl/svcl.exe
                /svv/SoundVolumeView.exe
    */

    let audio_device = media.active_audio().expect("no audio device found");
    let micro_device = media.active_micro().expect("no micro device found");

    println!("Audio device: '{}', Microphone: '{}'", &audio_device.name, &micro_device.name);
    println!("Audio volume: {}%", audio_device.get_volume()?);

    audio_device.set_volume(10)?;
    println!("Audio volume: {}%", audio_device.get_volume()?);
    
    Ok(())
}
```

## Feedback:

> This library distributed under the [MIT](https://github.com/fuderis/rs-pc-remote/blob/main/LICENSE.md) license.

You can contact me via GitHub or send a message to my telegram [@fuderis](https://t.me/fuderis).
This library is actively evolving, and your suggestions and feedback are always welcome!
