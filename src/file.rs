use stumpless_sys::*;

use std::error::Error;
use std::ffi::CString;

use crate::StumplessError;
use crate::Target;

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
