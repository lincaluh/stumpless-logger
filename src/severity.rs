use stumpless_sys::*;

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
