use crate::{
    aopa::AudioObjPropAddress,
    error::Error,
    mscope::PropertyScope,
    mselector::{AudioDevPropSelector, PropertySelector},
};
use coreaudio_sys::{
    AudioDeviceID, AudioObjectGetPropertyData, AudioObjectGetPropertyDataSize,
    AudioObjectHasProperty, AudioObjectSetPropertyData, AudioStreamID, UInt32,
};
use std::{
    ffi::c_void,
    mem::size_of,
    ptr::{self, addr_of, null},
};

const CHANNEL_CHECK_FAILS: usize = 3;
const DEVICE_NAME_LEN: usize = 256;

pub fn get_property_complex<T>(
    device_id: AudioDeviceID,
    property: AudioObjPropAddress,
    data_ptr: *mut T,
    data_size: usize,
) -> Result<(), Error> {
    let ptr = data_ptr.cast::<c_void>();
    let mut data_size = u32::try_from(data_size)?;

    let status = unsafe {
        AudioObjectGetPropertyData(device_id, &property.into(), 0, null(), &mut data_size, ptr)
    };

    if status != 0 {
        return Err(status.into());
    }

    Ok(())
}

pub fn get_property<T: Default + Sized>(
    device_id: AudioDeviceID,
    property: AudioObjPropAddress,
) -> Result<T, Error> {
    let mut data = T::default();
    let data_ptr = ptr::from_mut(&mut data);
    let data_size = size_of::<T>();

    get_property_complex(device_id, property, data_ptr, data_size)?;

    Ok(data)
}

pub fn set_property<T>(
    device_id: AudioDeviceID,
    property: AudioObjPropAddress,
    value: &T,
) -> Result<(), Error> {
    let ptr = addr_of!(*value) as *mut c_void;
    let data_size = size_of::<T>() as UInt32;

    let status = unsafe {
        AudioObjectSetPropertyData(device_id, &property.into(), 0, null(), data_size, ptr)
    };

    if status != 0 {
        return Err(status.into());
    }
    Ok(())
}

pub fn has_property(device_id: AudioDeviceID, property: AudioObjPropAddress) -> bool {
    let ret = unsafe { AudioObjectHasProperty(device_id, &property.into()) };
    ret != 0
}

pub fn get_valid_channels(id: AudioDeviceID, scope: PropertyScope) -> Vec<u32> {
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

pub fn get_property_data_size(
    id: AudioDeviceID,
    address: AudioObjPropAddress,
) -> Result<usize, Error> {
    let mut size = 0u32;

    let status =
        unsafe { AudioObjectGetPropertyDataSize(id, &address.into(), 0, null(), &mut size) };

    if status != 0 {
        return Err(status.into());
    }

    Ok(usize::try_from(size)?)
}

pub fn get_device_name(id: AudioDeviceID, scope: PropertyScope) -> Result<String, Error> {
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

pub fn get_streams(id: AudioDeviceID, scope: PropertyScope) -> Result<u32, Error> {
    if !matches!(scope, PropertyScope::DEV_INPUT | PropertyScope::DEV_OUTPUT) {
        return Err(Error::UnexpectedParam);
    }

    let address = AudioObjPropAddress::new(PropertySelector::DEV_STREAMS, scope);

    let stream_count_bytes: usize = get_property::<u32>(id, address)?.try_into()?;
    let stream_count = stream_count_bytes / size_of::<AudioStreamID>();

    Ok(stream_count.try_into()?)
}
