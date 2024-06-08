use super::AudioDevice;
use std::ops::Deref;

#[derive(Debug)]
pub struct AudioInputDevice(pub(crate) AudioDevice);

impl Deref for AudioInputDevice {
    type Target = AudioDevice;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
