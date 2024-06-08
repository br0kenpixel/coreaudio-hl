use coreaudio_hl::devices::get_all_outputs;

fn main() -> Result<(), coreaudio_hl::error::Error> {
    let devices = get_all_outputs()?;

    for device in devices {
        println!("{}", device.name());
    }

    Ok(())
}
