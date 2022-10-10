use regex::Regex;
use std::error::Error;

mod entry;
pub use crate::entry::{add_entry, Entry};

mod error;
pub use crate::error::{perror, StumplessError};

mod facility;
pub use crate::facility::Facility;

mod file;
pub use crate::file::FileTarget;

mod severity;
pub use crate::severity::Severity;

mod target;
pub use crate::target::{add_message, Target};

#[cfg(feature = "journald")]
mod journald;
#[cfg(feature = "journald")]
pub use crate::journald::JournaldTarget;

#[cfg(feature = "network")]
mod network;
#[cfg(feature = "network")]
pub use crate::network::NetworkTarget;

#[cfg(feature = "socket")]
mod socket;
#[cfg(feature = "socket")]
pub use crate::socket::SocketTarget;

#[cfg(feature = "wel")]
mod wel;
#[cfg(feature = "wel")]
pub use crate::wel::{add_default_wel_event_source, WelTarget};

pub fn prival_from_string(priority: &str) -> Result<i32, Box<dyn Error>> {
    match priority.parse::<i32>() {
        Ok(prival) => { if prival >= 0 && prival <= 191 { return Ok(prival); } }
        _ => {}
    }

    let priority_re = Regex::new(r"^(\w+).(\w+)$").unwrap();
    match priority_re.captures(priority) {
        Some(caps) => {
            let facility;
            match caps.get(1).unwrap().as_str() {
                "kern" => { facility = 0; }
                "user" => { facility = 1; }
                "mail" => { facility = 2; }
                "daemon" => { facility = 3; }
                "auth" | "security" => { facility = 4; }
                "syslog" => { facility = 5; }
                "lpr" => { facility = 6; }
                "news" => { facility = 7; }
                "uucp" => { facility = 8; }
                "cron" => { facility = 9; }
                "authpriv" => { facility = 10; }
                "ftp" => { facility = 11; }
                "ntp" => { facility = 12; }
                "local0" => { facility = 16; }
                "local1" => { facility = 17; }
                "local2" => { facility = 18; }
                "local3" => { facility = 19; }
                "local4" => { facility = 20; }
                "local5" => { facility = 21; }
                "local6" => { facility = 22; }
                "local7" => { facility = 23; }
                _ => { return Err(Box::new(StumplessError)); }
            }

            let severity;
            match caps.get(2).unwrap().as_str() {
                "emerg" | "panic" => { severity = 0; }
                "alert" => { severity = 1; }
                "crit" => { severity = 2; }
                "err" | "error" => { severity = 3; }
                "warning" | "warn" => { severity = 4; }
                "notice" => { severity = 5; }
                "info" => { severity = 6; }
                "debug" => { severity = 7; }
                _ => { return Err(Box::new(StumplessError)); }
            }

            return Ok((facility * 8) + severity);
        }
        None => {
            return Err(Box::new(StumplessError));
        }
    }
}