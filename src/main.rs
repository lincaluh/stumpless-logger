use clap::{arg, command};
use stumpless::{Entry, Facility, FileTarget, Severity};

#[cfg(feature = "journald")]
use stumpless::JournaldTarget;

#[cfg(feature = "socket")]
use stumpless::SocketTarget;

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
        .arg(arg!(-'u' --"socket" [socket] "Write to the provided socket, or /dev/log if none is provided.").required(false))
        .arg(arg!(-'l' --"log-file" <file> "Log the entry to the given file.").required(false))
        .arg(arg!(message: <message> "The message to send in the log entry.").multiple_values(true))
        .get_matches();

    let message = cli_matches
        .values_of("message")
        .unwrap()
        .collect::<String>();

    let entry = Entry::new(Facility::User, Severity::Alert, "app_name", "msgid", &message).expect("entry creation failed!");

    if cli_matches.is_present("log-file") {
        let log_filename = cli_matches.value_of("log-file").unwrap();
        let file_target = FileTarget::new(log_filename).unwrap();
        file_target
            .add_entry(&entry)
            .expect("logging to the file failed!");
    }

    #[cfg(feature = "journald")]
    if cli_matches.is_present("journald") {
        let journald_target = JournaldTarget::new().unwrap();
        journald_target
            .add_entry(&entry)
            .expect("logging to journald failed!");
    }

    #[cfg(not(feature = "journald"))]
    if cli_matches.is_present("journald") {
        eprintln!("journald logging not enabled, ignoring --journald option");
    }

    #[cfg(feature = "socket")]
    if cli_matches.is_present("socket") {
        let socket_name = cli_matches.value_of("socket").unwrap();
        let socket_target = SocketTarget::new(socket_name).unwrap();
        socket_target
            .add_entry(&entry)
            .expect("logging to socket failed!");
    }

    #[cfg(not(feature = "socket"))]
    if cli_matches.is_present("socket") {
        eprintln!("socket logging not enabled, ignoring --socket option");
    }
}
