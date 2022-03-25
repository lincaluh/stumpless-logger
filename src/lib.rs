use stumpless_sys::*;

use std::error::Error;
use std::ffi::CString;
use std::fmt;

mod facility;
pub use crate::facility::Facility;

mod severity;
pub use crate::severity::Severity;

mod file;
pub use crate::file::FileTarget;

#[cfg(feature = "journald")]
mod journald;
#[cfg(feature = "journald")]
pub use crate::socket::JournaldTarget;

#[cfg(feature = "network")]
mod network;
#[cfg(feature = "network")]
pub use crate::socket::NetworkTarget;

#[cfg(feature = "socket")]
mod socket;
#[cfg(feature = "socket")]
pub use crate::socket::SocketTarget;

#[cfg(feature = "wel")]
mod wel;
#[cfg(feature = "wel")]
pub use crate::socket::WelTarget;

#[derive(Debug, Clone)]
pub struct StumplessError;

impl Error for StumplessError {}

impl fmt::Display for StumplessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "stumpless error encountered!")
    }
}

pub fn perror(prefix: &str) {
    let c_prefix = CString::new(prefix).expect("couldn't make a C string");

    unsafe { stumpless_perror(c_prefix.as_ptr()) }
}

pub struct Entry {
    entry: *mut stumpless_entry,
}

impl Entry {
    pub fn new(
        facility: Facility,
        severity: Severity,
        app_name: &str,
        msgid: &str,
        message: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let c_app_name = CString::new(app_name)?;
        let c_msgid = CString::new(msgid)?;
        let c_message = CString::new(message)?;
        let new_entry = unsafe {
            stumpless_new_entry_str(
                (facility as u32).try_into().unwrap(),
                (severity as u32).try_into().unwrap(),
                c_app_name.as_ptr(),
                c_msgid.as_ptr(),
                c_message.as_ptr(),
            )
        };

        if new_entry.is_null() {
            Err(Box::new(StumplessError))
        } else {
            Ok(Entry { entry: new_entry })
        }
    }
}

pub trait Target {
    fn get_pointer(&self) -> *mut stumpless_target;
}

pub fn add_entry(target: &impl Target, entry: &Entry) -> Result<u32, Box<dyn Error>> {
    let add_result = unsafe {
        stumpless_add_entry(target.get_pointer(), entry.entry)
    };

    if add_result >= 0 {
        Ok(add_result.try_into().unwrap())
    } else {
        Err(Box::new(StumplessError))
    }
}

pub fn add_message(target: &impl Target, message: &str) -> Result<u32, Box<dyn Error>> {
    let c_message = CString::new(message)?;

    let add_result = unsafe {
        stumpless_add_message_str(target.get_pointer(), c_message.as_ptr())
    };

    if add_result >= 0 {
        Ok(add_result.try_into().unwrap())
    } else {
        Err(Box::new(StumplessError))
    }
}
