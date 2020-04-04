# Move The Sheriff

🤠 ↕️↔️

_**Move The Sheriff**_ is an action-adventure stealth game for terminal emulators.

see [1.0.0](https://github.com/austinpray/move-the-sheriff/tree/1.0.0) for the original moveTheSheriff.js

## Usage

```
move-the-sheriff 2.0.0
Austin Pray <austin@austinpray.com>

USAGE:
    move-the-sheriff [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --server <server>         [default: 127.0.0.1:6142]
    -u, --username <username>     [default: anonymous]
```

## Building

Move The Sheriff is ncurses-based. You will need [ncurses header files](https://github.com/gyscos/Cursive/wiki/Install-ncurses).

```bash
cargo build
```

You can also run it with cargo:

```bash
cargo run -- --help
```