//! You can consider these scratchings to be my slow descent into madness.  The ultimate goal of
//! this project is to provide fluent interaction between Rust and the .NET ecosystem.
//!
//! Currently, the focus is mostly upon providing a clean API to host the Common Language Runtime (CLR)
//! as intended.  The scope of the project is currently limited to dotnet-core, with most of the
//! testing being done in a unix environment.

pub mod host;