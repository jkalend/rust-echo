# Rust-echo
The echo UNIX utility written in Rust

# Usage
```shell
$ echo <TEXT> [-n]
```

 - `<TEXT>` Stands for any input text 

Options:
  - `-n`             Omits the final newline  
  - `-h`, `--help`     Print help  
  -  `-V`, `--version`  Print version  

# Build
```shell
$ cargo build
```

## Manpage
The build script also creates a `.1` file to be used as a manpage. This file can be found in the `target` directory.

## License
This project is licensed under ISC license.
