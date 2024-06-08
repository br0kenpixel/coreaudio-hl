use crate::{
    aopa::AudioObjPropAddress,
    error::Error,
    internals,
    mscope::AudioDevPropScope,
    mselector::{AudioHwPropSelector, PropertySelector},
};
use coreaudio_sys::{kAudioObjectSystemObject, AudioDeviceID};

#[derive(Debug)]
pub struct AudioOutputDevice {
    device_id: AudioDeviceID,
    valid_channels: Box<[u32]>,
    name: Box<str>,
}

impl AudioOutputDevice {
    pub fn with_id(id: AudioDeviceID) -> Result<Self, Error> {
        let valid_channels = internals::get_valid_channels(id, AudioDevPropScope::Output);
        let name = internals::get_device_name(id, AudioDevPropScope::Output)?;

        Ok(Self {
            device_id: id,
            valid_channels: valid_channels.into(),
            name: name.into(),
        })
    }

    pub fn get_default() -> Result<Self, Error> {
        Self::with_id(Self::default_device_id()?)
    }

    /*** --- Device property getters --- ***/

    pub fn avg_volume(&self) -> Result<f32, Error> {
        let volumes = self
            .valid_channels
            .iter()
            .map(|ch| self.volume_for_channel(*ch))
            .collect::<Result<Vec<_>, Error>>()?;

        let avg = volumes.iter().sum::<f32>() / volumes.len() as f32;

        Ok(avg)
    }

    pub fn volume_for_channel(&self, ch: u32) -> Result<f32, Error> {
        self.get_property(AudioObjPropAddress::new_with_element(
            PropertySelector::DEV_VOLUME_SCALAR,
            AudioDevPropScope::Output,
            ch,
        ))
    }

    pub fn muted(&self) -> Result<bool, Error> {
        self.get_property::<i32>(AudioObjPropAddress::new(
            PropertySelector::DEV_MUTE,
            AudioDevPropScope::Output,
        ))
        .map(|result| result == 1)
    }

    /*** --- Utils --- ***/

    pub fn get_property<T: Default + Sized>(&self, prop: AudioObjPropAddress) -> Result<T, Error> {
        internals::get_property(self.device_id, prop)
    }

    pub fn is_default(&self) -> Result<bool, Error> {
        Ok(Self::default_device_id()? == self.device_id)
    }

    pub fn has_property(&self, prop: AudioObjPropAddress) -> bool {
        internals::has_property(self.device_id, prop)
    }

    /*** --- Struct getters --- ***/

    pub fn channels(&self) -> &[u32] {
        &self.valid_channels
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /*** --- Private --- ***/

    fn default_device_id() -> Result<AudioDeviceID, Error> {
        internals::get_property(
            kAudioObjectSystemObject,
            AudioObjPropAddress::new(
                PropertySelector::Hardware(AudioHwPropSelector::DefaultOutputDevice),
                AudioDevPropScope::Output,
            ),
        )
    }
}

impl Default for AudioOutputDevice {
    fn default() -> Self {
        Self::get_default().unwrap()
    }
}
