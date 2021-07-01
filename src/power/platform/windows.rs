use wasmer_enumset::EnumSetType;
use winapi::shared::minwindef::DWORD;
use winapi::um::errhandlingapi;
use winapi::um::minwinbase::REASON_CONTEXT;
use winapi::um::winnt::{HANDLE, POWER_REQUEST_TYPE};
use winapi::um::{handleapi, winbase, winnt};

use crate::power::POWER_DEFAULT_MESSAGE;

const REQUEST_TYPES: &[LockType] = &[
    LockType::AutomaticSuspend,
    LockType::ManualSuspend,
    LockType::DisplaySuspend,
];

#[derive(Debug)]
pub enum Error {
    FailedToCreateRequest(DWORD),
    FailedToLock(DWORD),
}

#[derive(Debug, EnumSetType)]
pub enum LockType {
    /// Automatic suspension (managed by the system idle timer)
    AutomaticSuspend,
    /// Manual suspension
    ManualSuspend,
    /// Display suspension
    DisplaySuspend,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FailedToCreateRequest(_) => write!(f, "failed to create power request"),
            Self::FailedToLock(_) => write!(f, "failed lock operation"),
        }
    }
}

#[derive(Debug)]
pub struct Lock {
    request: PowerRequest,
}

impl Lock {
    fn new() -> Result<Self, Error> {
        let request =
            PowerRequest::new(POWER_DEFAULT_MESSAGE).map_err(Error::FailedToCreateRequest)?;

        let mut failed: Option<(LockType, DWORD)> = None;

        for lock_type in REQUEST_TYPES.iter() {
            let result =
                unsafe { winbase::PowerSetRequest(request.0, Self::request_type(*lock_type)) };
            if result == 0 {
                failed = Some((*lock_type, unsafe { errhandlingapi::GetLastError() }));
                break;
            }
        }

        match failed {
            Some((failed_type, err_code)) => {
                for lock_type in REQUEST_TYPES.iter().take_while(|t| **t != failed_type) {
                    unsafe {
                        winbase::PowerClearRequest(request.0, Self::request_type(*lock_type))
                    };
                }
                Err(Error::FailedToLock(err_code))
            }
            None => Ok(Self { request }),
        }
    }

    fn request_type(lock_type: LockType) -> POWER_REQUEST_TYPE {
        match lock_type {
            LockType::AutomaticSuspend => winnt::PowerRequestSystemRequired,
            LockType::ManualSuspend => winnt::PowerRequestAwayModeRequired,
            LockType::DisplaySuspend => winnt::PowerRequestDisplayRequired,
        }
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        for lock_type in REQUEST_TYPES.iter() {
            unsafe { winbase::PowerClearRequest(self.request.0, Self::request_type(*lock_type)) };
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

#[derive(Debug)]
struct PowerRequest(HANDLE);

impl PowerRequest {
    fn new(msg: &str) -> Result<Self, DWORD> {
        let mut context: REASON_CONTEXT = REASON_CONTEXT {
            Version: winnt::POWER_REQUEST_CONTEXT_VERSION,
            Flags: winnt::POWER_REQUEST_CONTEXT_SIMPLE_STRING,
            ..Default::default()
        };
        let mut text: Vec<u16> = msg.encode_utf16().collect();
        unsafe { *context.Reason.SimpleReasonString_mut() = text.as_mut_ptr() };

        let request = unsafe { winbase::PowerCreateRequest(&mut context) };
        if request.is_null() {
            Err(unsafe { errhandlingapi::GetLastError() })
        } else {
            Ok(Self(request))
        }
    }
}

impl Drop for PowerRequest {
    fn drop(&mut self) {
        unsafe { handleapi::CloseHandle(self.0) };
    }
}
