use coreaudio_hl::{
    aopa::AudioObjPropAddress,
    devices::AudioOutputDevice,
    mscope::PropertyScope,
    mselector::{AudioDevPropSelector, PropertySelector},
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let output = AudioOutputDevice::get_default()?;
    let mut _address = AudioObjPropAddress::new(
        PropertySelector::DEV_VOLUME_SCALAR,
        PropertyScope::DEV_OUTPUT,
    );

    // or...
    let mut address = AudioObjPropAddress::new(
        PropertySelector::Device(AudioDevPropSelector::VolumeScalar),
        PropertyScope::DEV_OUTPUT,
    );

    for channel in output.channels() {
        address.set_element(*channel);

        let volume = output.get_property::<String>(address)?;
        println!("Volume on channel #{channel}: {:.02}%", volume);
    }

    Ok(())
}
