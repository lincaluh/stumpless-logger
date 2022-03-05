#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("stumpless_bindings.rs");

use std::error::Error;
use std::ffi::CString;

pub enum Facility {
    Kern = stumpless_facility_STUMPLESS_FACILITY_KERN as isize,
    User = stumpless_facility_STUMPLESS_FACILITY_USER as isize,
}

pub struct Entry {
    entry: *mut stumpless_entry,
}

pub struct FileTarget {
    target: *mut stumpless_target,
}

impl FileTarget {
    pub fn new(filename: &str) -> Result<Self, Box<dyn Error>> {
        let c_filename = CString::new(filename)?;
        let file_target = unsafe { stumpless_open_file_target(c_filename.as_ptr()) };

        if file_target.is_null() {
            panic!("ah crap, stumpless couldn't open that file!");
        }

        Ok(FileTarget {
            target: file_target,
        })
    }

    pub fn add_message(&self, message: &str) -> Result<u32, Box<dyn Error>> {
        let c_message = CString::new(message)?;

        unsafe {
            stumpless_add_message_str(self.target, c_message.as_ptr());
        }

        Ok(1)
    }

    // TODO: need to add the destructor
}
