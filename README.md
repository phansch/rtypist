[![Travis CI](https://travis-ci.org/phansch/rtypist.svg?branch=master)](https://travis-ci.org/phansch/rtypist)
[![Circle CI](https://circleci.com/gh/phansch/rtypist/tree/master.svg?style=svg)](https://circleci.com/gh/phansch/rtypist/tree/master)

# Rtypist

A re-implementation of GNUtypist in Rust.. at some point.


## Development

You will need the ncurses library installed.
See [here](https://github.com/gyscos/Cursive/wiki/Install-ncurses) for more information.

Run `cargo build && cargo test && cargo run`.

### Clippy

Clippy is set up as an optional dependency because you will need the latest nightly for it to compile and run.

Enable Clippy by running:

    cargo build --features "clippy"

Run clippy with the following two commands:

    cargo rustc --bin main --features clippy -- -Z no-trans -Z extra-plugins=clippy
    cargo rustc --lib --features clippy -- -Z no-trans -Z extra-plugins=clippy
