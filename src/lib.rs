use stumpless_sys::*;

use std::error::Error;
use std::ffi::CString;
use std::fmt;

pub enum Facility {
    Kernel = stumpless_facility_STUMPLESS_FACILITY_KERN as isize,
    User = stumpless_facility_STUMPLESS_FACILITY_USER as isize,
    Mail = stumpless_facility_STUMPLESS_FACILITY_MAIL as isize,
    Daemon = stumpless_facility_STUMPLESS_FACILITY_DAEMON as isize,
    Auth = stumpless_facility_STUMPLESS_FACILITY_AUTH as isize,
    Syslog = stumpless_facility_STUMPLESS_FACILITY_SYSLOG as isize,
    Lpr = stumpless_facility_STUMPLESS_FACILITY_LPR as isize,
    News = stumpless_facility_STUMPLESS_FACILITY_NEWS as isize,
    Uucp = stumpless_facility_STUMPLESS_FACILITY_UUCP as isize,
    Cron = stumpless_facility_STUMPLESS_FACILITY_CRON as isize,
    Auth2 = stumpless_facility_STUMPLESS_FACILITY_AUTH2 as isize,
    FTP = stumpless_facility_STUMPLESS_FACILITY_FTP as isize,
    NTP = stumpless_facility_STUMPLESS_FACILITY_NTP as isize,
    Audit = stumpless_facility_STUMPLESS_FACILITY_AUDIT as isize,
    Alert = stumpless_facility_STUMPLESS_FACILITY_ALERT as isize,
    Cron2 = stumpless_facility_STUMPLESS_FACILITY_CRON2 as isize,
    Local0 = stumpless_facility_STUMPLESS_FACILITY_LOCAL0 as isize,
    Local1 = stumpless_facility_STUMPLESS_FACILITY_LOCAL1 as isize,
    Local2 = stumpless_facility_STUMPLESS_FACILITY_LOCAL2 as isize,
    Local3 = stumpless_facility_STUMPLESS_FACILITY_LOCAL3 as isize,
    Local4 = stumpless_facility_STUMPLESS_FACILITY_LOCAL4 as isize,
    Local5 = stumpless_facility_STUMPLESS_FACILITY_LOCAL5 as isize,
    Local6 = stumpless_facility_STUMPLESS_FACILITY_LOCAL6 as isize,
    Local7 = stumpless_facility_STUMPLESS_FACILITY_LOCAL7 as isize,
}

pub enum Severity {
    Emergency = stumpless_severity_STUMPLESS_SEVERITY_EMERG as isize,
    Alert = stumpless_severity_STUMPLESS_SEVERITY_ALERT as isize,
    Critical = stumpless_severity_STUMPLESS_SEVERITY_CRIT as isize,
    Error = stumpless_severity_STUMPLESS_SEVERITY_ERR as isize,
    Warning = stumpless_severity_STUMPLESS_SEVERITY_WARNING as isize,
    Notice = stumpless_severity_STUMPLESS_SEVERITY_NOTICE as isize,
    Info = stumpless_severity_STUMPLESS_SEVERITY_INFO as isize,
    Debug = stumpless_severity_STUMPLESS_SEVERITY_DEBUG as isize,
}

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

#[cfg(feature = "socket")]
pub struct SocketTarget {
    target: *mut stumpless_target,
}

#[cfg(feature = "socket")]
impl SocketTarget {
    pub fn new(socket_name: &str) -> Result<Self, Box<dyn Error>> {
        let c_socket_name = CString::new(socket_name)?;
        let socket_target =
            unsafe { stumpless_open_socket_target(c_socket_name.as_ptr(), std::ptr::null()) };

        if socket_target.is_null() {
            Err(Box::new(StumplessError))
        } else {
            Ok(SocketTarget {
                target: socket_target,
            })
        }
    }
}

#[cfg(feature = "socket")]
impl Target for SocketTarget {
    fn get_pointer(&self) -> *mut stumpless_target {
        self.target
    }
}

#[cfg(feature = "socket")]
impl Drop for SocketTarget {
    fn drop(&mut self) {
        unsafe {
            stumpless_close_socket_target(self.target);
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
