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

impl crate::power::Lock for Lock {
    type Error = Error;
    type Lock = Lock;

    fn new() -> Result<Lock, Self::Error> {
        Lock::new()
    }
}
