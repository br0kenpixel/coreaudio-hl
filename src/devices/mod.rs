use crate::{
    aopa::AudioObjPropAddress,
    error::Error,
    internals::{get_property_complex, get_property_data_size},
    mscope::PropertyScope,
    mselector::PropertySelector,
};
use coreaudio_sys::{kAudioObjectSystemObject, AudioDeviceID};
pub use output::AudioOutputDevice;
use std::mem::size_of;

mod input;
mod output;

pub fn get_all_devices() -> Result<(Vec<usize>, Vec<AudioOutputDevice>), Error> {
    let address =
        AudioObjPropAddress::new(PropertySelector::HW_ALL_DEVICES, PropertyScope::OBJ_GLOBAL);

    let size = get_property_data_size(kAudioObjectSystemObject, address)?;
    let n_devices = size / size_of::<AudioDeviceID>();
    let mut devices: Vec<AudioDeviceID> = vec![0; n_devices];

    get_property_complex(
        kAudioObjectSystemObject,
        address,
        devices.as_mut_ptr(),
        size,
    )?;

    let inputs = Vec::new();
    let mut outputs = Vec::new();

    for device_id in devices {
        if let Ok(output_dev) = AudioOutputDevice::with_id(device_id) {
            outputs.push(output_dev);
        }

        // TODO: Inputs
    }

    Ok((inputs, outputs))
}

pub fn get_all_outputs() -> Result<Vec<AudioOutputDevice>, Error> {
    get_all_devices().map(|result| result.1)
}
