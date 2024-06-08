use coreaudio_hl::devices::get_all_devices;

fn main() -> Result<(), coreaudio_hl::error::Error> {
    let devices = get_all_devices()?;

    for device in devices {
        println!("{}", device.name());
        println!("{:?}", device.input_channels());
        println!("{:?}", device.output_channels());
        println!("{:?}", device.input_streams());
        println!("{:?}", device.output_streams());
    }

    Ok(())
}
