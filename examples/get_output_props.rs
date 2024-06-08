use coreaudio_hl::devices::AudioOutputDevice;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let output = AudioOutputDevice::get_default()?;
    let name = output.name();
    let mute = output.muted()?;

    println!("Name: {name}");
    println!("Muted? {mute}");

    for channel in output.channels() {
        let volume = output.volume_for_channel(*channel)?;

        println!("Volume on channel #{channel}: {:.02}%", volume);
    }

    println!("Volume: {:.02}%", output.avg_volume()?);

    Ok(())
}
