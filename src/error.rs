use coreaudio_sys::{AudioDeviceID, OSStatus};
use std::{num::TryFromIntError, str::Utf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("OS error code: {0}")]
    Raw(OSStatus),
    #[error("Failed to parse UTF-8 bytes: {0}")]
    Utf8Error(#[from] Utf8Error),
    #[error("Invalid scalar volume value: {0} ")]
    InvalidVolume(f32),
    #[error("Failed to convert integer values: {0}")]
    IntConversion(#[from] TryFromIntError),
    #[error("Unexpected parameter")]
    UnexpectedParam,
    #[error("Device ID links to an input device")]
    NotOutput,
    #[error("Unable to determine device type of device {0}")]
    UnknownDeviceType(AudioDeviceID),
    #[error("Callback registration failed")]
    CallbackRegister,
}

impl From<i32> for Error {
    fn from(value: i32) -> Self {
        Self::Raw(value)
    }
}
