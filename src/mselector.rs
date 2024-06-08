use coreaudio_sys::{
    kAudioDevicePropertyDeviceName, kAudioDevicePropertyMute, kAudioDevicePropertyVolumeScalar,
    kAudioHardwarePropertyDefaultInputDevice, kAudioHardwarePropertyDefaultOutputDevice,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AudioDevPropSelector {
    VolumeScalar,
    Mute,
    Name,
}

impl From<AudioDevPropSelector> for u32 {
    fn from(value: AudioDevPropSelector) -> Self {
        match value {
            AudioDevPropSelector::VolumeScalar => kAudioDevicePropertyVolumeScalar,
            AudioDevPropSelector::Mute => kAudioDevicePropertyMute,
            AudioDevPropSelector::Name => kAudioDevicePropertyDeviceName,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AudioHwPropSelector {
    DefaultInputDevice,
    DefaultOutputDevice,
}

impl From<AudioHwPropSelector> for u32 {
    fn from(value: AudioHwPropSelector) -> Self {
        match value {
            AudioHwPropSelector::DefaultInputDevice => kAudioHardwarePropertyDefaultInputDevice,
            AudioHwPropSelector::DefaultOutputDevice => kAudioHardwarePropertyDefaultOutputDevice,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PropertySelector {
    Device(AudioDevPropSelector),
    Hardware(AudioHwPropSelector),
}

impl PropertySelector {
    pub const DEV_VOLUME_SCALAR: Self = Self::Device(AudioDevPropSelector::VolumeScalar);
    pub const DEV_MUTE: Self = Self::Device(AudioDevPropSelector::Mute);
    pub const DEV_NAME: Self = Self::Device(AudioDevPropSelector::Name);

    pub const HW_DEFAULT_INPUT_DEV: Self = Self::Hardware(AudioHwPropSelector::DefaultInputDevice);
    pub const HW_DEFAULT_OUTPUT_DEV: Self =
        Self::Hardware(AudioHwPropSelector::DefaultOutputDevice);
}

impl From<PropertySelector> for u32 {
    fn from(value: PropertySelector) -> Self {
        match value {
            PropertySelector::Device(dev) => dev.into(),
            PropertySelector::Hardware(hw) => hw.into(),
        }
    }
}
