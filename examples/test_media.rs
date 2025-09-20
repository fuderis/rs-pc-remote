use pc_remote::{ Result, media::Media };
use macron::path;

fn main() -> Result<()> {
    let media = Media::new(path!("bin/nirsoft"))?;

    let audio_device = media.active_audio().expect("no audio device found");
    let micro_device = media.active_micro().expect("no micro device found");

    println!("Audio device: '{}', Microphone: '{}'", &audio_device.name, &micro_device.name);
    println!("Audio volume: {}%", audio_device.get_volume()?);

    audio_device.set_volume(10)?;
    println!("Audio volume: {}%", audio_device.get_volume()?);
    
    Ok(())
}
