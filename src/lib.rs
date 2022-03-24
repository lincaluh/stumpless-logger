use stumpless_sys::*;

use std::error::Error;
use std::ffi::CString;
use std::fmt;

mod facility;
pub use crate::facility::Facility;

mod severity;
pub use crate::severity::Severity;

#[cfg(feature = "socket")]
mod socket;
#[cfg(feature = "socket")]
pub use crate::socket::SocketTarget;

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

pub struct FileTarget {
    target: *mut stumpless_target,
}

impl FileTarget {
    pub fn new(filename: &str) -> Result<Self, Box<dyn Error>> {
        let c_filename = CString::new(filename)?;
        let file_target = unsafe { stumpless_open_file_target(c_filename.as_ptr()) };

        if file_target.is_null() {
            Err(Box::new(StumplessError))
        } else {
            Ok(FileTarget {
                target: file_target,
            })
        }
    }
}

impl Target for FileTarget {
    fn get_pointer(&self) -> *mut stumpless_target {
        self.target
    }
}

impl Drop for FileTarget {
    fn drop(&mut self) {
        unsafe {
            stumpless_close_file_target(self.target);
        }
    }
}

#[cfg(feature = "journald")]
pub struct JournaldTarget {
    target: *mut stumpless_target,
}

#[cfg(feature = "journald")]
impl JournaldTarget {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let target_name = CString::new("stumpless-cli")?;
        let journald_target = unsafe { stumpless_open_journald_target(target_name.as_ptr()) };

        if journald_target.is_null() {
            Err(Box::new(StumplessError))
        } else {
            Ok(JournaldTarget {
                target: journald_target,
            })
        }
    }
}

#[cfg(feature = "journald")]
impl Target for JournaldTarget {
    fn get_pointer(&self) -> *mut stumpless_target {
        self.target
    }
}

#[cfg(feature = "journald")]
impl Drop for JournaldTarget {
    fn drop(&mut self) {
        unsafe {
            stumpless_close_journald_target(self.target);
        }
    }
}

#[cfg(feature = "network")]
pub struct NetworkTarget {
    target: *mut stumpless_target,
}

#[cfg(feature = "network")]
impl NetworkTarget {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let target_name = CString::new("stumpless-cli")?;
        let network_target = unsafe { stumpless_open_network_target(target_name.as_ptr(), 0, 0) };

        if network_target.is_null() {
            Err(Box::new(StumplessError))
        } else {
            Ok(NetworkTarget {
                target: network_target,
            })
        }
    }
}

#[cfg(feature = "network")]
impl Target for NetworkTarget {
    fn get_pointer(&self) -> *mut stumpless_target {
        self.target
    }
}

#[cfg(feature = "network")]
impl Drop for NetworkTarget {
    fn drop(&mut self) {
        unsafe {
            stumpless_close_network_target(self.target);
        }
    }
}

#[cfg(feature = "wel")]
pub struct WelTarget {
    target: *mut stumpless_target,
}

#[cfg(feature = "wel")]
impl WelTarget {
    pub fn new(log_name: &str) -> Result<Self, Box<dyn Error>> {
        let c_log_name = CString::new(log_name)?;
        let wel_target =
            unsafe { stumpless_open_local_wel_target(c_log_name.as_ptr()) };

        if wel_target.is_null() {
            Err(Box::new(StumplessError))
        } else {
            Ok(WelTarget {
                target: wel_target,
            })
        }
    }
}

#[cfg(feature = "wel")]
impl Target for WelTarget {
    fn get_pointer(&self) -> *mut stumpless_target {
        self.target
    }
}

#[cfg(feature = "wel")]
impl Drop for WelTarget {
    fn drop(&mut self) {
        unsafe {
            stumpless_close_wel_target(self.target);
        }
    }
}
