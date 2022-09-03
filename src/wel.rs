use stumpless_sys::{
    stumpless_add_default_wel_event_source, stumpless_close_wel_target,
    stumpless_open_local_wel_target, stumpless_target,
};

use std::error::Error;
use std::ffi::CString;

use crate::StumplessError;
use crate::Target;

pub struct WelTarget {
    target: *mut stumpless_target,
}

impl WelTarget {
    pub fn new(log_name: &str) -> Result<Self, Box<dyn Error>> {
        let c_log_name = CString::new(log_name)?;
        let wel_target = unsafe { stumpless_open_local_wel_target(c_log_name.as_ptr()) };

        if wel_target.is_null() {
            Err(Box::new(StumplessError))
        } else {
            Ok(WelTarget { target: wel_target })
        }
    }
}

impl Target for WelTarget {
    fn get_pointer(&self) -> *mut stumpless_target {
        self.target
    }
}

impl Drop for WelTarget {
    fn drop(&mut self) {
        unsafe {
            stumpless_close_wel_target(self.target);
        }
    }
}

pub fn add_default_wel_event_source() -> Result<u32, Box<dyn Error>> {
    let add_result = unsafe { stumpless_add_default_wel_event_source() };

    if add_result == 0 {
        Ok(add_result.try_into().unwrap())
    } else {
        Err(Box::new(StumplessError))
    }
}
