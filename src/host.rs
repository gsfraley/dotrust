extern crate libloading as libl;

use std::collections::HashMap;
use std::ffi::CString;
use std::io;
use std::os::raw::{c_char, c_int, c_uint, c_void};


// -- Private utility functions -- //
fn to_c_str<T: Into<Vec<u8>>>(t: T) -> *const c_char {
    CString::new(t).unwrap().as_ptr() as *const c_char
}


// -- Model the functions we'll dynamically load -- //
pub type CoreClrInitializeFn = unsafe extern fn(
    *const c_char,
    *const c_char,
    c_int,
    *const *const c_char,
    *const *const c_char,
    *const *const c_void,
    *const c_uint) -> c_int;

pub type CoreClrShutdownFn = unsafe extern fn(*const c_void, c_uint) -> c_int;

pub type CoreClrShutdown2Fn = unsafe extern fn(*const c_void, c_uint, *const c_int) -> c_int;


// -- Model the CLR -- //
pub struct CoreClr {
    host_handle: *const c_void,
    domain_id: c_uint
}

impl CoreClr {
    fn library() -> libl::Result<libl::Library> {
        libl::Library::new("/usr/local/share/dotnet/shared/Microsoft.NETCore.App/2.0.0")
    }

    pub fn init(
        exe_path: &str,
        app_domain_friendly_name: &str,
        properties_option: Option<HashMap<&str, &str>>) -> libl::Result<CoreClr>
    {
        // Create the host handle and its ref
        let host_handle = 0 as *const c_void;
        let host_handle_ref = &host_handle as *const *const c_void;
        
        // Create the domain id and its ref
        let domain_id = 0 as c_uint;
        let domain_id_ref = &domain_id as *const c_uint;

        // Raw C string refs from exe_path and app_domain_friendly_name slices
        let exe_path_raw = to_c_str(exe_path);
        let app_domain_friendly_name_raw = to_c_str(app_domain_friendly_name);

        // Either use the provided option or a blank hashmap
        let properties = properties_option.unwrap_or_else(HashMap::new);

        // Count for upcoming vecs
        let properties_count = properties.len() as c_int;

        // Collect property keys into a vec
        let properties_keys: Vec<*const c_char> = properties.keys()
                .map(|&k| to_c_str(k))
                .collect();
        
        // Collect equivalent property values into another vec
        let properties_values: Vec<*const c_char> = properties.values()
                .map(|&v| to_c_str(v))
                .collect();

        // Grab refs of the two to pass into the actual function call
        let properties_keys_ref = &properties_keys[0] as *const *const c_char;
        let properties_values_ref = &properties_values[0] as *const *const c_char;

        // Open up an unsafe block for actually loading functions from the CLR libs
        unsafe {
            let coreclr_library = CoreClr::library()?;
            let coreclr_initialize: libl::Symbol<CoreClrInitializeFn> = coreclr_library.get(b"coreclr_initialize")?;

            // Initialize the CLR
            match coreclr_initialize(
                exe_path_raw,
                app_domain_friendly_name_raw,
                properties_count,
                properties_keys_ref,
                properties_values_ref,
                host_handle_ref,
                domain_id_ref)
            {
                // If healthy exit code, return a model of the CLR
                0 => Ok(CoreClr {
                    host_handle: host_handle,
                    domain_id: domain_id
                }),
                // Else panic
                _ => panic!("Failed to initialize")
            }
        }
    }

    pub fn shutdown(self: Self) -> io::Result<()> {
        unsafe {
            let coreclr_library = CoreClr::library()?;
            let coreclr_shutdown_fn: libl::Symbol<CoreClrShutdownFn> = coreclr_library.get(b"coreclr_shutdown")?;

            // Shutdown the CLR
            match coreclr_shutdown_fn(self.host_handle, self.domain_id) {
                // If healthy exit code, return unit
                0 => Ok(()),
                // Else panic
                _ => panic!("Failed to shutdown")
            }
        }
    }

    pub fn shutdown_2(self: Self) -> io::Result<c_int> {
        let latched_exit_code = -1 as c_int;
        let latched_exit_code_ref = &latched_exit_code as *const c_int;

        unsafe {
            let coreclr_library = CoreClr::library()?;
            let coreclr_shutdown_2_fn: libl::Symbol<CoreClrShutdown2Fn> = coreclr_library.get(b"coreclr_shutdown_2")?;

            // Shutdown the CLR
            match coreclr_shutdown_2_fn(self.host_handle, self.domain_id, latched_exit_code_ref) {
                // If healthy exit code, return the resulting exit code
                0 => Ok(latched_exit_code),
                // Else panic
                _ => panic!("Failed to shutdown")
            }
        }
    }
}