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
