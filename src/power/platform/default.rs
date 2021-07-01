#[derive(Debug)]
pub enum Error {
    UnsupportedOperatingSystem,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "unsupported operating system")
    }
}

#[derive(Debug)]
pub struct Lock;

impl Lock {
    fn new() -> Result<Self, Error> {
        Err(Error::UnsupportedOperatingSystem)
    }
}

impl crate::power::Lock for Lock {
    type Error = Error;
    type Lock = Lock;

    fn new() -> Result<Lock, Self::Error> {
        Lock::new()
    }
}
