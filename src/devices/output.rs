use crate::{
    aopa::AudioObjPropAddress,
    error::Error,
    internals,
    mscope::PropertyScope,
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
        // Check if the device has output streams
        if internals::get_streams(id, PropertyScope::DEV_OUTPUT)? < 1 {
            return Err(Error::NotOutput);
        }

        let valid_channels = internals::get_valid_channels(id, PropertyScope::DEV_OUTPUT);
        let name = internals::get_device_name(id, PropertyScope::DEV_OUTPUT)?;

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
            PropertyScope::DEV_OUTPUT,
            ch,
        ))
    }

    pub fn muted(&self) -> Result<bool, Error> {
        self.get_property::<i32>(AudioObjPropAddress::new(
            PropertySelector::DEV_MUTE,
            PropertyScope::DEV_OUTPUT,
        ))
        .map(|result| result == 1)
    }

    /*** --- Device property setters --- ***/

    pub fn set_mute(&self, mute: bool) -> Result<(), Error> {
        let mut address =
            AudioObjPropAddress::new(PropertySelector::DEV_MUTE, PropertyScope::DEV_OUTPUT);
        let mute = mute as u32;

        for channel in self.valid_channels.iter() {
            address.set_element(*channel);

            if self.set_property(address, &mute).is_err() {
                // try the master channel
                address.set_element(0);
                return self.set_property(address, &mute);
            }
        }

        Ok(())
    }

    pub fn set_volume(&self, vol: f32) -> Result<(), Error> {
        if !(0.00..1.0).contains(&vol) {
            return Err(Error::InvalidVolume(vol));
        }

        let mut address = AudioObjPropAddress::new(
            PropertySelector::DEV_VOLUME_SCALAR,
            PropertyScope::DEV_OUTPUT,
        );

        for channel in self.channels().iter() {
            address.set_element(*channel);
            self.set_property(address, &vol)?;
        }

        Ok(())
    }

    /*** --- Utils --- ***/

    pub fn get_property<T: Default + Sized>(&self, prop: AudioObjPropAddress) -> Result<T, Error> {
        internals::get_property(self.device_id, prop)
    }

    pub fn set_property<T: Default + Sized>(
        &self,
        prop: AudioObjPropAddress,
        value: &T,
    ) -> Result<(), Error> {
        internals::set_property(self.device_id, prop, value)
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
                PropertyScope::DEV_OUTPUT,
            ),
        )
    }
}

impl Default for AudioOutputDevice {
    fn default() -> Self {
        Self::get_default().unwrap()
    }
}
