use crate::power::POWER_DEFAULT_MESSAGE;
use core_foundation::base::TCFType;
use core_foundation::string::{CFString, CFStringRef};
use mach::kern_return::kern_return_t;

type IOReturn = kern_return_t;
type IOPMAssertionLevel = u32;
type IOPMAssertionId = u32;

const K_IOPMASSERTION_LEVEL_ON: IOPMAssertionLevel = 255;

extern "C" {
    pub fn IOPMAssertionCreateWithName(
        assertion_type: CFStringRef,
        assertion_level: IOPMAssertionLevel,
        assertion_name: CFStringRef,
        assertion_id: &mut IOPMAssertionId,
    ) -> IOReturn;

    pub fn IOPMAssertionRelease(assertion_id: IOPMAssertionId) -> IOReturn;
}

#[derive(Debug)]
pub enum Error {}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Lock {
    request: IOPMAssertionId,
}

impl Lock {
    pub fn new() -> Result<Self, Error> {
        let k_iopmassertion_type_prevent_system_sleep: CFString =
            CFString::from_static_string("NoDisplaySleepAssertion");
        let mut id: IOPMAssertionId = 0;
        let reason_cf = CFString::new(POWER_DEFAULT_MESSAGE);
        unsafe {
            let _ret = IOPMAssertionCreateWithName(
                k_iopmassertion_type_prevent_system_sleep.as_concrete_TypeRef(),
                K_IOPMASSERTION_LEVEL_ON,
                reason_cf.as_concrete_TypeRef(),
                &mut id,
            );
        }
        Ok(Lock { request: id })
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        unsafe {
            let _res = IOPMAssertionRelease(self.request);
        }
    }
}

unsafe impl Send for Lock {}

impl crate::power::Lock for Lock {
    type Error = Error;
    type Lock = Lock;

    fn new() -> Result<Lock, Self::Error> {
        Lock::new()
    }
}
