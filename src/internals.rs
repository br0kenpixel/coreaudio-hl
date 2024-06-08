use crate::{
    aopa::AudioObjPropAddress,
    error::Error,
    mscope::AudioDevPropScope,
    mselector::{AudioDevPropSelector, PropertySelector},
};
use coreaudio_sys::{AudioDeviceID, AudioObjectGetPropertyData, AudioObjectHasProperty, UInt32};
use std::{
    ffi::c_void,
    mem::size_of,
    ptr::{addr_of, null},
};

const CHANNEL_CHECK_FAILS: usize = 3;
const DEVICE_NAME_LEN: usize = 256;

pub fn get_property<T: Default + Sized>(
    device_id: AudioDeviceID,
    property: AudioObjPropAddress,
) -> Result<T, Error> {
    let result_container = T::default();
    let ptr = addr_of!(result_container) as *mut c_void;
    let mut data_size = size_of::<T>() as UInt32;

    let status = unsafe {
        AudioObjectGetPropertyData(device_id, &property.into(), 0, null(), &mut data_size, ptr)
    };

    if status != 0 {
        return Err(status.into());
    }

    Ok(result_container)
}

pub fn has_property(device_id: AudioDeviceID, property: AudioObjPropAddress) -> bool {
    let ret = unsafe { AudioObjectHasProperty(device_id, &property.into()) };
    ret != 0
}

pub fn get_valid_channels(id: AudioDeviceID, scope: AudioDevPropScope) -> Vec<u32> {
    let mut result = Vec::new();
    let mut address = AudioObjPropAddress::new(
        PropertySelector::Device(AudioDevPropSelector::VolumeScalar),
        scope,
    );
    let mut failures = 0;

    while failures < CHANNEL_CHECK_FAILS {
        if has_property(id, address) {
            result.push(address.element());
        } else {
            failures += 1;
        }
        address.set_element(address.element() + 1);
    }

    result
}

pub fn get_device_name(id: AudioDeviceID, scope: AudioDevPropScope) -> Result<String, Error> {
    let address = AudioObjPropAddress::new(PropertySelector::DEV_NAME, scope);
    let mut name_buf = [0u8; DEVICE_NAME_LEN];
    let ptr = name_buf.as_mut_ptr().cast::<c_void>();
    let mut data_size = DEVICE_NAME_LEN as UInt32;

    let status =
        unsafe { AudioObjectGetPropertyData(id, &address.into(), 0, null(), &mut data_size, ptr) };

    if status != 0 {
        return Err(Error::Raw(status));
    }

    let name = std::str::from_utf8(&name_buf)?;

    Ok(name.into())
}
