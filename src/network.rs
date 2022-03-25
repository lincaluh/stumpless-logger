use stumpless_sys::*;

use std::error::Error;
use std::ffi::CString;

use crate::StumplessError;
use crate::Target;

pub struct NetworkTarget {
    target: *mut stumpless_target,
}

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

impl Target for NetworkTarget {
    fn get_pointer(&self) -> *mut stumpless_target {
        self.target
    }
}

impl Drop for NetworkTarget {
    fn drop(&mut self) {
        unsafe {
            stumpless_close_network_target(self.target);
        }
    }
}
