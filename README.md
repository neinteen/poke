# Poke

### Description

Inspired by the classic [touch](https://en.wikipedia.org/wiki/Touch_(command)) command.

Please take a look at the [ROADMAP](https://github.com/neinteen/poke/issues/1) to understand more about what this project is going to be,
and which features are currently working.

Thanks to all crates used for saving me a whole lot of time.
* [clap](https://crates.io/crates/clap)
* [filetime](https://crates.io/crates/filetime)
* [thiserror](https://crates.io/crates/thiserror)
* [chrono](https://crates.io/crates/chrono)
* [chrono_english](https://crates.io/crates/chrono_english)

### Installation

`cargo install --force poke`

### Usage

```rust
USAGE:
    poke.exe [OPTIONS] <FILES>...

ARGS:
    <FILES>...    the file(s) to be modified

OPTIONS:
    -a
            change only the access time

    -b
            delete given file(s)

    -c, --no-create
            if not found, don't create new file(s)

    -d, --date <DATE>
            use this date string instead of current time. The Supported formats are: "1/01/2001
            00:00", "1 jan(uary) 2001 0am", "last friday 2pm" and so on. for more information:
            https://crates.io/crates/chrono-english

    -h, --help
            Print help information

    -m
            change only the modification time

    -r, --reference-file <REFERENCE_FILE>
            use this files time, instead of current time

    -V, --version
            Print version information
```

### Compatibility

The current version (0.1.3) is only tested on windows, but should support Linux/Mac OS. In the future I will test Linux builds in a vm.