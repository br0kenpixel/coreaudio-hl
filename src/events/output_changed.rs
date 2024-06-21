use crate::{
    aopa::AudioObjPropAddress, devices::AudioDevice, error::Error, mscope::PropertyScope,
    mselector::PropertySelector,
};
use coreaudio_sys::{
    kAudioObjectSystemObject, AudioObjectAddPropertyListener, AudioObjectID,
    AudioObjectPropertyAddress, AudioObjectRemovePropertyListener, OSStatus, UInt32,
};
use std::{ffi::c_void, ptr::null_mut, sync::RwLock};

type Callback = fn(AudioDevice);
const ADDRESS: AudioObjPropAddress = AudioObjPropAddress::new(
    PropertySelector::HW_DEFAULT_OUTPUT_DEV,
    PropertyScope::OBJ_GLOBAL,
);
static CALLBACK: RwLock<Option<Callback>> = RwLock::new(None);

pub fn register(callback: Callback) -> Result<(), Error> {
    let status = unsafe {
        AudioObjectAddPropertyListener(
            kAudioObjectSystemObject,
            &ADDRESS.into(),
            Some(callback_wrapper),
            null_mut(),
        )
    };

    if status != 0 {
        return Err(status.into());
    }

    let mut slot = CALLBACK.write().unwrap();
    slot.replace(callback);

    Ok(())
}

pub fn unregister() -> Result<(), Error> {
    let status = unsafe {
        AudioObjectRemovePropertyListener(
            kAudioObjectSystemObject,
            &ADDRESS.into(),
            Some(callback_wrapper),
            null_mut(),
        )
    };

    if status != 0 {
        return Err(status.into());
    }

    let mut slot = CALLBACK.write().unwrap();
    slot.take();

    Ok(())
}

unsafe extern "C" fn callback_wrapper(
    in_obj_id: AudioObjectID,
    _in_number_addresses: UInt32,
    _in_addresses: *const AudioObjectPropertyAddress,
    _in_client_data: *mut c_void,
) -> OSStatus {
    let device = AudioDevice::from_id(in_obj_id).unwrap();
    let hl_clbk = unsafe { CALLBACK.read().unwrap_unchecked().unwrap_unchecked() };

    hl_clbk(device);

    1
}
