//! The ultimate goal of this project is to provide fluent interaction between Rust and the .NET
//! ecosystem. Currently, the focus is mostly upon providing a clean API to host the Common Language
//! Runtime (CLR) as intended.

#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
#[macro_use]
extern crate com_rs as com;

pub mod host;