use crate::{
    aopa::AudioObjPropAddress,
    error::Error,
    internals::{self, get_property_complex, get_property_data_size},
    mscope::PropertyScope,
    mselector::PropertySelector,
};
use coreaudio_sys::{kAudioObjectSystemObject, AudioDeviceID};
pub use output::AudioOutputDevice;
use std::mem::size_of;

mod input;
mod output;

#[derive(Debug)]
pub struct AudioDevice {
    device_id: AudioDeviceID,
    name: Box<str>,
    input_channels: Box<[u32]>,
    output_channels: Box<[u32]>,
    input_streams: u32,
    output_streams: u32,
}

impl AudioDevice {
    pub fn from_id(id: AudioDeviceID) -> Result<Self, Error> {
        let name = internals::get_device_name(id, PropertyScope::DEV_OUTPUT)?;
        let input_channels = internals::get_valid_channels(id, PropertyScope::DEV_INPUT);
        let output_channels = internals::get_valid_channels(id, PropertyScope::DEV_OUTPUT);
        let input_streams = internals::get_streams(id, PropertyScope::DEV_INPUT)?;
        let output_streams = internals::get_streams(id, PropertyScope::DEV_OUTPUT)?;

        Ok(Self {
            device_id: id,
            name: name.into(),
            input_channels: input_channels.into(),
            output_channels: output_channels.into(),
            input_streams,
            output_streams,
        })
    }

    pub fn default_input() -> Result<Self, Error> {
        Self::from_id(Self::default_input_device_id()?)
    }

    pub fn default_output() -> Result<AudioOutputDevice, Error> {
        let output_dev = Self::from_id(Self::default_output_device_id()?)?;

        Ok(unsafe { output_dev.as_output().unwrap_unchecked() })
    }

    pub fn as_output(self) -> Option<AudioOutputDevice> {
        if !self.is_output() {
            return None;
        }

        Some(AudioOutputDevice(self))
    }

    pub const fn id(&self) -> AudioDeviceID {
        self.device_id
    }

    pub const fn name(&self) -> &str {
        &self.name
    }

    pub const fn input_channels(&self) -> &[u32] {
        &self.input_channels
    }

    pub const fn output_channels(&self) -> &[u32] {
        &self.output_channels
    }

    pub const fn is_input(&self) -> bool {
        !self.input_channels.is_empty()
    }

    pub const fn is_output(&self) -> bool {
        !self.output_channels.is_empty()
    }

    pub const fn is_multi(&self) -> bool {
        !self.input_channels.is_empty() && !self.output_channels.is_empty()
    }

    pub const fn input_streams(&self) -> u32 {
        self.input_streams
    }

    pub const fn output_streams(&self) -> u32 {
        self.output_streams
    }

    fn default_input_device_id() -> Result<AudioDeviceID, Error> {
        internals::get_property(
            kAudioObjectSystemObject,
            AudioObjPropAddress::new(
                PropertySelector::HW_DEFAULT_INPUT_DEV,
                PropertyScope::DEV_INPUT,
            ),
        )
    }

    pub(crate) fn default_output_device_id() -> Result<AudioDeviceID, Error> {
        internals::get_property(
            kAudioObjectSystemObject,
            AudioObjPropAddress::new(
                PropertySelector::HW_DEFAULT_OUTPUT_DEV,
                PropertyScope::DEV_OUTPUT,
            ),
        )
    }
}

pub fn get_all_devices() -> Result<Vec<AudioDevice>, Error> {
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

    devices.into_iter().map(AudioDevice::from_id).collect()
}
