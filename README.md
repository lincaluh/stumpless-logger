[![Apache 2.0 License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)


**An enhanced command line logging utility.**


## Key Features
The stumpless logger aims to be a replacement for and improvement over the
traditional `logger` utility. It is written with
[Rust](https://www.rust-lang.org/) and
[Stumpless](https://github.com/goatshriek/stumpless), and offers a number of
improvements over legacy tool, including:

  * more logging target options (file, Windows Event Log)
  * log to multiple destinations with a single invocation
  * portable and available on both Windows as well as Linux


## Send Your Logs Anywhere
The stumpless logger supports all of the target types that Stumpless provides,
which include everything `logger` has and then some.


#### Stdout
By default, logs are written to stdout. If you invoke the logger with no
arguments, this is what you will get.

```sh
example will go here
```

If you want to be explicit about including stdout output, then you can include
the `<need to choose>` flag. This is useful when you want to print your log to
stdout as well as send it to other targets.

```sh
example will go here
```

Of course, adding a message will result in that message being printed in the
log:

```sh
example will go here
```

#### Sockets
```sh
example will go here
```


#### Files
```sh
example will go here
```

#### Network
```sh
example will go here
```


#### Journald
```sh
example will go here
```


#### Structured Data
```sh
examples for structured data and fields will go here
```


## Differences Between `stumpless` and `logger`
This logger is _not_ written as a drop-in replacement for other `logger`
implementations. There are differences that arise from decisions made for
simplicity, performance, or necessity. Here are the differences that are
relevant to you if you're already familiar with or using other loggers.

  * The default output with no arguments is `stdout` instead of `/dev/log`.
  * The time quality structured data element is not included (pending Stumpless
    implementation of this feature).
  * The following flags/modes of operation are not supported:
    * `--rfc3164` for the RFC 3164 BSD syslog format of messages
