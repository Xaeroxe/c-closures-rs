# c-closures [![Crates listing](https://img.shields.io/crates/v/c-closures.svg)](https://crates.io/crates/c-closures) [![Travis](https://travis-ci.com/Xaeroxe/c-closures-rs.svg?branch=master)](https://travis-ci.com/Xaeroxe/c-closures-rs)

[Documentation](https://docs.rs/c-closures/)

Provides a general purpose way for Rust closures to cross an FFI boundary into C/C++, enabling cross language functional programming.

# Contributing

I welcome contributions and alterations to this project! [Here's some info to help you get started.](https://help.github.com/articles/about-pull-requests/)

- If you would consider your change substantial open an issue on the issues tab so we can discuss it before you build it.
- If you're fixing a bug please provide a unit test for the bug fixed so we don't do it again.
- If applicable, new features should have some basic unit tests added too.
- Please format your code with the most recent stable release of rustfmt before submitting your PR.
- I don't have a regular release schedule, if you want something you've added put on crates.io right away make sure to
bump the version number for the project in your pull request.

# no_std

This crate provides a `no_std` feature to alter the generated code, however since using `no_std` is still very unstable no testing of any kind is done on this.
Fixes to this are welcome, but no guarantees are provided.
