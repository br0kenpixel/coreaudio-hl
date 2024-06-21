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

The library doesn't expose every possible property selector, but if you need something, it's easy to add.

Features:
- [x] Get the default input/output device
    - [x] Get/set volume
        - [x] Per-channel
    - [x] Get/set mute state
    - [x] Get/set name
    - [x] Get channels
- [X] Attach callbacks for events
    - [ ] Volume change
    - [x] Default input/output device change

Example:
```rust
use coreaudio_hl::devices::AudioDevice;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let device = AudioDevice::default_output()?;
    let name = device.name();
    let mute = device.muted()?;

    println!("Name: {name}");
    println!("Muted? {mute}");

    for channel in device.output_channels() {
        let volume = device.volume_for_channel(*channel)?;

        println!("Volume on channel #{channel}: {:.02}%", volume);
    }

    println!("Volume: {:.02}%", device.avg_volume()?);

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
    let device = AudioDevice::default_output()?;
    let mut _address = AudioObjPropAddress::new(
        PropertySelector::DEV_VOLUME_SCALAR,
        PropertyScope::DEV_OUTPUT,
    );

    // or...
    let mut address = AudioObjPropAddress::new(
        PropertySelector::Device(AudioDevPropSelector::VolumeScalar),
        PropertyScope::DEV_OUTPUT,
    );

    for channel in device.output_channels() {
        address.set_element(*channel);

        let volume = device.get_property::<String>(address)?;
        println!("Volume on channel #{channel}: {:.02}%", volume);
    }

    Ok(())
}
```
Note that this is **highly** not recommended. You should always use proper getters, as specified in the first example.