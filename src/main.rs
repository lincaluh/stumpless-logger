use clap::{arg, command, Arg, ValueSource};
use itertools::Itertools;
use stumpless::{add_entry, Entry, Facility, FileTarget, Severity};

#[cfg(feature = "journald")]
use stumpless::JournaldTarget;

#[cfg(feature = "network")]
use stumpless::NetworkTarget;

#[cfg(feature = "socket")]
use stumpless::SocketTarget;

#[cfg(feature = "wel")]
use stumpless::{add_default_wel_event_source, WelTarget};

fn main() {
    let cli_matches = command!()
        .arg(
            arg!(-'i' --"id" [id] "Log the PID of the stumpless process in each entry.")
                .long_help(
                    "When the optional argument id is specified, then it is used instead of the
command's PID. It's recommended to set this to a single value in scripts that
send multiple messages, for example the script's own process id.

Note that some logging infrastructure (for example systemd when listening on
/dev/log) may overwrite this value, for example with the one derived from the
connecting socket.",
                )
                .multiple_values(false),
        )
        .arg(arg!(-'f' --"file" "Log the contents of the file instead of reading from stdin or message arg.").required(false))
        .arg(arg!(-'j' --"journald" "Log the entry to the journald system.").required(false))
        .arg(arg!(-'u' --"socket" [socket] "Log to the provided socket, or /dev/log if none is provided.").required(false))
        .arg(arg!(-'l' --"log-file" <file> "Log the entry to the given file.").required(false))
        .arg(arg!(-'c' --"tcp4" <server> "Send the entry to the given server using TCP over IPv4.").required(false))
        .arg(
            Arg::new("windows-event-log")
                .short('w')
                .long("windows-event-log")
                .takes_value(true)
                .value_name("log")
                .help("Log to the Windows Event Log provided.")
                .default_missing_value("Stumpless")
                .min_values(0)
                .require_equals(true)
                .required(false))
        .arg(arg!(--"install-wel-default-source" "Installs the stumpless default Windows Event Log source.")
                .long_help(
                        "Having the event source information installed is required for the
Event Viewer to properly display events logged to it. This only needs to happen
once, and can be done after the events themselves are logged with no loss of
information. This option requires privileges to access and modify the Windows
Registry to function properly.",
                )
                .required(false)
        )
        .arg(Arg::new("message").help("The message to send in the log entry.").multiple_values(true).required_unless("install-wel-default-source"))
        .get_matches();

    #[cfg(feature = "wel")]
    if cli_matches.is_present("install-wel-default-source") {
        add_default_wel_event_source()
            .expect("adding the default Windows Event Log source failed!");
    }

    #[cfg(not(feature = "wel"))]
    if cli_matches.is_present("install-wel-default-source") {
        eprintln!("Windows Event Log functionality is not enabled, ignoring --install-wel-default-source option")
    }

    if cli_matches.occurrences_of("message") == 0 {
        // we are all done if there is no message to log
        return;
    }

    let message_iterator = cli_matches.values_of("message").unwrap();

    let message = Itertools::intersperse(message_iterator, " ").collect::<String>();

    let entry = Entry::new(
        Facility::User,
        Severity::Alert,
        "app_name",
        "msgid",
        &message,
    )
    .expect("entry creation failed!");

    if cli_matches.is_present("log-file") {
        let log_filename = cli_matches.value_of("log-file").unwrap();
        match FileTarget::new(log_filename) {
            Err(_error) => stumpless::perror("opening the file target failed"),
            Ok(target) => {
                if let Err(_error) = add_entry(&target, &entry) {
                    stumpless::perror("logging to the file target failed");
                }
            }
        };
    }

    #[cfg(feature = "journald")]
    if cli_matches.is_present("journald") {
        let journald_target = JournaldTarget::new().unwrap();
        add_entry(&journald_target, &entry).expect("logging to journald failed!");
    }

    #[cfg(not(feature = "journald"))]
    if cli_matches.is_present("journald") {
        eprintln!("journald logging not enabled, ignoring --journald option");
    }

    #[cfg(feature = "socket")]
    if cli_matches.is_present("socket") {
        let socket_name = cli_matches.value_of("socket").unwrap();
        let socket_target = SocketTarget::new(socket_name).unwrap();
        add_entry(&socket_target, &entry).expect("logging to socket failed!");
    }

    #[cfg(not(feature = "socket"))]
    if cli_matches.is_present("socket") {
        eprintln!("socket logging not enabled, ignoring --socket option");
    }

    #[cfg(feature = "wel")]
    if cli_matches.value_source("windows-event-log") == Some(ValueSource::CommandLine) {
        let wel_log_name = cli_matches.value_of("windows-event-log").unwrap();
        let wel_target = WelTarget::new(wel_log_name).unwrap();
        add_entry(&wel_target, &entry).expect("logging to the Windows Event Log failed!");
    }

    #[cfg(not(feature = "wel"))]
    if cli_matches.is_present("windows-event-log") {
        eprintln!("Windows Event Log logging is not enabled, ignoring --windows-event-log option");
    }
}
