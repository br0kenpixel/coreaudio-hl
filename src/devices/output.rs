use super::AudioDevice;
use crate::{
    aopa::AudioObjPropAddress, error::Error, internals, mscope::PropertyScope,
    mselector::PropertySelector,
};
use std::ops::Deref;

#[derive(Debug)]
pub struct AudioOutputDevice(pub(crate) AudioDevice);

impl AudioOutputDevice {
    pub fn avg_volume(&self) -> Result<f32, Error> {
        let volumes = self
            .output_channels()
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

    pub fn set_mute(&self, mute: bool) -> Result<(), Error> {
        let mut address =
            AudioObjPropAddress::new(PropertySelector::DEV_MUTE, PropertyScope::DEV_OUTPUT);
        let mute = mute as u32;

        for channel in self.output_channels().iter() {
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

        for channel in self.output_channels().iter() {
            address.set_element(*channel);
            self.set_property(address, &vol)?;
        }

        Ok(())
    }

    pub fn downgrade(self) -> AudioDevice {
        self.0
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
        Ok(AudioDevice::default_output_device_id()? == self.device_id)
    }

    pub fn has_property(&self, prop: AudioObjPropAddress) -> bool {
        internals::has_property(self.device_id, prop)
    }
}

impl Deref for AudioOutputDevice {
    type Target = AudioDevice;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
