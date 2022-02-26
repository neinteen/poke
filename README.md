# Poke

---

##### Description

A Simple program that aims to replicate the functionality of [touch]([touch (command) - Wikipedia](https://en.wikipedia.org/wiki/Touch_(command))), and expand on it.

Thanks to: [clap](https://crates.io/crates/clap), [filetime](https://crates.io/crates/filetime) and [thiserror](https://crates.io/crates/thiserror) for saving me a whole lot of time.

##### Features

- [x] Create files
  
- [x] Delete files
  
- [x] Change Access/Modification time (supports reference file)

- [] Change Creation time (only in a specific case)
  
- [ ] Support for special syntax
  
  > poke file{3}.rs
  > 
  > poke some_file ./some_dir/other_file.rs
  

##### Installation

`cargo install --force poke`

##### Commands

```rust
USAGE:
    poke.exe [OPTIONS] <files>...

ARGS:
    <files>...

OPTIONS:
    -a                                  change only access time.
    -b                                  deletes files. incompatible with any other command.
    -c, --no-create                     do not create any files.
    -d, --date <date_string>            parse time string and use it instead of current time. (NOT
                                        IMPLEMENTED)
    -h, --help                          Print help information
    -m                                  change only modification time.
    -r, --reference <reference_file>    use this file's times instead of current time.
    -V, --version                       Print version information
```

##### Compatibility

The current version (0.1.1) is only tested on windows, but should support Linux/Mac OS. In the future I will test Linux builds in a vm.