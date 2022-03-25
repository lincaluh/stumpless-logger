use stumpless_sys::stumpless_perror;

use std::error::Error;
use std::ffi::CString;
use std::fmt;

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
