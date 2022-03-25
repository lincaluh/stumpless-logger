use stumpless_sys::{stumpless_add_entry, stumpless_entry, stumpless_new_entry_str};

use std::error::Error;
use std::ffi::CString;
use crate::facility::Facility;
use crate::severity::Severity;
use crate::error::StumplessError;
use crate::target::Target;

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
