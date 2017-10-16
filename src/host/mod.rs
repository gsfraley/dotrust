//! Hosting the CLR with Rust

pub mod unix;
pub mod windows;

#[cfg(unix)]
pub type CoreClr = unix::UnixCoreClr;

#[cfg(test)]
mod tests {
    use super::CoreClr;

    #[test]
    fn init_and_shutdown() {
        let coreclr = CoreClr::init("", "", None).unwrap();
        coreclr.shutdown();
    }
}