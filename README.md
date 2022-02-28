# Poke

### Description

A Simple program that aims to replicate the functionality of [touch](https://en.wikipedia.org/wiki/Touch_(command)), and expand on it.

Thanks to all crates used for saving me a whole lot of time.

Please take a look at the [ROADMAP](https://github.com/neinteen/poke/issues/1) to understand more about what this project is going to be,
and which features are currently working.

### Installation

`cargo install --force poke`

### Commands

```rust
Poke 
neinteen
A Simple program that aims to replicate the functionality of touch, and expand on it

USAGE:
    poke.exe [OPTIONS] <FILES>...

ARGS:
    <FILES>...    single or multiple files

OPTIONS:
    -a
            change only the access time

    -b
            delete given files

    -c, --no-create
            don't create new file, if the given file wasn't found

    -d, --date <DATE>
            use this time string instead of current time.

    -h, --help
            Print help information

    -m
            change only the modification time

    -r, --reference-file <REFERENCE_FILE>
            use this files time, instead of current time.

    -t, --timestamp <TIMESTAMP>
            use this timestamp, instead of current time.
```

### Compatibility

The current version (0.1.2) is only tested on windows, but should support Linux/Mac OS. In the future I will test Linux builds in a vm.