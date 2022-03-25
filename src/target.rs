use std::error::Error;
use std::ffi::CString;
use stumpless_sys::{stumpless_add_message_str, stumpless_target};
use crate::error::StumplessError;

pub trait Target {
    fn get_pointer(&self) -> *mut stumpless_target;
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
