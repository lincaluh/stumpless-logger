use stumpless_sys::*;

use std::error::Error;
use std::ffi::CString;

use crate::StumplessError;
use crate::Target;

pub struct SocketTarget {
    target: *mut stumpless_target,
}

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

impl Target for SocketTarget {
    fn get_pointer(&self) -> *mut stumpless_target {
        self.target
    }
}

impl Drop for SocketTarget {
    fn drop(&mut self) {
        unsafe {
            stumpless_close_socket_target(self.target);
        }
    }
}
