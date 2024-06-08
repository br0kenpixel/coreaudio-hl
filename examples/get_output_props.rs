use coreaudio_hl::devices::AudioDevice;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let device = AudioDevice::default_output()?;
    let name = device.name();
    let mute = device.muted()?;

    println!("Name: {name}");
    println!("Muted? {mute}");

    for channel in device.output_channels() {
        let volume = device.volume_for_channel(*channel)?;

        println!("Volume on channel #{channel}: {:.02}%", volume);
    }

    println!("Volume: {:.02}%", device.avg_volume()?);

    Ok(())
}
