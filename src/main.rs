use clap::{arg, command};
use stumpless::FileTarget;

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
        .arg(arg!(-'l' --"log-file" <file> "Log the entry to the given file.").required(false))
        .arg(arg!(message: <message> "The message to send in the log entry.").multiple_values(true))
        .get_matches();

    let message = cli_matches
        .values_of("message")
        .unwrap()
        .collect::<String>();

    if cli_matches.is_present("log-file") {
        let log_filename = cli_matches.value_of("log-file").unwrap();
        let file_target = FileTarget::new(log_filename).unwrap();
        file_target.add_message(&message).expect("logging to the file failed!");
    }
}
