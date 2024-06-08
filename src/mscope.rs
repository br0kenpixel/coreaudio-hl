use coreaudio_sys::{
    kAudioDevicePropertyScopeInput, kAudioDevicePropertyScopeOutput,
    kAudioDevicePropertyScopePlayThrough,
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
