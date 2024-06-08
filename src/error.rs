use coreaudio_sys::OSStatus;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("OS error code: {0}")]
    Raw(OSStatus),
    #[error("Failed to parse UTF-8 bytes: {0}")]
    Utf8Error(#[from] Utf8Error),
}

impl From<i32> for Error {
    fn from(value: i32) -> Self {
        Self::Raw(value)
    }
}
