//! Hosting the CLR with Rust

use std::io;

#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
pub mod unix;

trait ClrHost {
    fn get_app_domain_id(self: &Self) -> io::Result<i32>;
    fn stop(self: Self) -> io::Result<()>;

    fn execute_assembly(self: &Self,
        assembly_path: &str,
        args: Vec<&str>) -> io::Result<i32>;

    unsafe fn create_delegate<T>(self: &Self,
        assembly_name: &str,
        class_name: &str,
        method_name: &str) -> io::Result<Box<T>>;
}

#[cfg(test)]
pub mod tests {
    use std::io;
    use std::ptr::null_mut;

    use super::ClrHost;

    struct HealthyClrHost;

    impl ClrHost for HealthyClrHost {
        fn get_app_domain_id(self: &Self) -> io::Result<i32> {
            Ok(8)
        }

        fn stop(self: Self) -> io::Result<()> {
            Ok(())
        }

        fn execute_assembly(self: &Self,
            _assembly_path: &str,
            _args: Vec<&str>) -> io::Result<i32> {
                Ok(0)
        }

        unsafe fn create_delegate<T>(self: &Self,
            _assembly_name: &str,
            _class_name: &str,
            _method_name: &str) -> io::Result<Box<T>> {
                let boxed_null: Box<T> = Box::from_raw(null_mut());
                Ok(boxed_null)
        }
    }

    #[test]
    fn it_works() {
        let healthy_clr_host = HealthyClrHost;
        healthy_clr_host.get_app_domain_id().unwrap();
        healthy_clr_host.execute_assembly("./my_assembly", vec![]).unwrap();

        let _delegate: Box<()> = unsafe {
            healthy_clr_host
                .create_delegate("./my_assembly", "Null", "value")
                .unwrap()
        };
        
        healthy_clr_host.stop().unwrap();
    }
}