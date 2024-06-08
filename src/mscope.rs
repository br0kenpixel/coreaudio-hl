use coreaudio_sys::{
    kAudioDevicePropertyScopeInput, kAudioDevicePropertyScopeOutput,
    kAudioDevicePropertyScopePlayThrough, kAudioObjectPropertyScopeGlobal,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AudioDevPropScope {
    Input,
    Output,
    PlayThrough,
}

impl From<AudioDevPropScope> for u32 {
    fn from(value: AudioDevPropScope) -> Self {
        match value {
            AudioDevPropScope::Input => kAudioDevicePropertyScopeInput,
            AudioDevPropScope::Output => kAudioDevicePropertyScopeOutput,
            AudioDevPropScope::PlayThrough => kAudioDevicePropertyScopePlayThrough,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AudioObjPropScope {
    Global,
}

impl From<AudioObjPropScope> for u32 {
    fn from(value: AudioObjPropScope) -> Self {
        match value {
            AudioObjPropScope::Global => kAudioObjectPropertyScopeGlobal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PropertyScope {
    Device(AudioDevPropScope),
    Object(AudioObjPropScope),
}

impl PropertyScope {
    pub const DEV_INPUT: Self = Self::Device(AudioDevPropScope::Input);
    pub const DEV_OUTPUT: Self = Self::Device(AudioDevPropScope::Output);

    pub const OBJ_GLOBAL: Self = Self::Object(AudioObjPropScope::Global);
}

impl From<PropertyScope> for u32 {
    fn from(value: PropertyScope) -> Self {
        match value {
            PropertyScope::Device(dev) => dev.into(),
            PropertyScope::Object(obj) => obj.into(),
        }
    }
}
