use wasmer_enumset::EnumSet;

use crate::power::LockType;

#[derive(Debug)]
pub struct InhibitionManager();

impl InhibitionManager {
    pub fn new() -> Result<Self, Error> {
        Ok(Self())
    }
}

impl crate::power::InhibitionManager for InhibitionManager {
    type Error = Error;
    type Lock = Lock;

    fn lock(&self, _types: EnumSet<LockType>) -> Result<Lock, Self::Error> {
        Lock::new()
    }
}

#[derive(Debug)]
pub enum Error {
    UnsupportedPlatform,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "unsupported platform")
    }
}

#[derive(Debug)]
pub struct Lock;

impl Lock {
    fn new() -> Result<Self, Error> {
        Err(Error::UnsupportedPlatform)
    }
}

impl crate::power::Lock for Lock {}
