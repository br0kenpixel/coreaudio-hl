# CoreAudio High-level wrapper for Rust
This library provides a safe and easy-to-use abstraction over Apple's CoreAudio C API for macOS. It provides several types that are internally safely converted to C types.

For example, instead of using `AudioObjectPropertyAddress`, you have a simple `AudioObjPropAddress` struct that only accepts specific values.
Take the following instance of `AudioObjPropAddress`:
```rust
let mut address = AudioObjPropAddress::new(
    PropertySelector::Device(AudioDevPropSelector::VolumeScalar),
    AudioDevPropScope::Output,
);
```
This is equivalent to the following:
```c
#include <AudioToolbox/AudioToolbox.h>

AudioObjectPropertyAddress propertyAddress = {
    kAudioDevicePropertyVolumeScalar,
    kAudioDevicePropertyScopeOutput,
    0
};
```

Features:
- [x] Get the default input/output device
    - [x] Get/set volume
        - [x] Per-channel
    - [x] Get/set mute state
    - [x] Get/set name
    - [x] Get channels
- [ ] Attach callbacks for events
    - [ ] Volume change
    - [ ] Default input/output device change

Example:
```rust
use coreaudio_hl::devices::AudioOutputDevice;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let output = AudioOutputDevice::get_default()?;
    let name = output.name(); // name is already stored internally
    let mute = output.muted()?; // calls AudioObjectGetPropertyData(), which could fail

    println!("Name: {name}");
    println!("Muted? {mute}");

    for channel in output.channels() {
        let volume = output.volume_for_channel(*channel)?;

        println!("Volume on channel #{channel}: {:.02}%", volume);
    }

    println!("Volume: {:.02}%", output.avg_volume()?);

    Ok(())
}
```
```
Name: WH-XB910N
Muted? false
Volume on channel #1: 0.37%
Volume on channel #2: 0.37%
Volume: 0.37%
```
Or, you can get an arbitrary property:
```rust
fn main() -> Result<(), Box<dyn Error>> {
    let output = AudioOutputDevice::get_default()?;

    let mut address = AudioObjPropAddress::new(
        PropertySelector::DEV_VOLUME_SCALAR,
        AudioDevPropScope::Output,
    );
    // or...
    let mut address = AudioObjPropAddress::new(
        PropertySelector::Device(AudioDevPropSelector::VolumeScalar),
        AudioDevPropScope::Output,
    );

    for channel in output.channels() {
        address.set_element(*channel);

        let volume = output.get_property::<f32>(address)?;
        println!("Volume on channel #{channel}: {:.02}%", volume);
    }

    Ok(())
}
```
Note that this is **highly** not recommended. You should always use proper getters, as specified in the first example.