use coreaudio_hl::{
    devices::AudioDevice,
    events::output_changed::{register, unregister},
};
use std::{thread::sleep, time::Duration};

fn main() {
    register(my_callback).unwrap();
    println!("Callback registered!");

    sleep(Duration::from_secs(10));

    unregister().unwrap();
    println!("Callback unregistered!");
}

fn my_callback(device: AudioDevice) {
    dbg!(device);
}
