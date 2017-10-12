extern crate libloading as libl;

use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint, c_void};

fn to_c_str<T: Into<Vec<u8>>>(t: T) -> *const c_char {
    CString::new(t).unwrap().as_ptr() as *const c_char
}

pub struct CoreClr {
    host_handle: *const c_void,
    domain_id: c_uint
}

pub type CoreClrInitializeFn = unsafe extern fn(
    *const c_char,
    *const c_char,
    c_int,
    *const *const c_char,
    *const *const c_char,
    *const *const c_void,
    *const c_uint) -> c_int;

pub type CoreClrShutdownFn = unsafe extern fn(*const c_void, c_uint);

impl CoreClr {
    pub fn init(
        exe_path: &str,
        app_domain_friendly_name: &str,
        properties_option: Option<HashMap<&str, &str>>) -> libl::Result<CoreClr>
    {
        let host_handle = 0 as *const c_void;
        let host_handle_ref = &host_handle as *const *const c_void;
        
        let domain_id = 0 as c_uint;
        let domain_id_ref = &domain_id as *const c_uint;

        let exe_path_raw = to_c_str(exe_path);
        let app_domain_friendly_name_raw = to_c_str(app_domain_friendly_name);

        let properties = properties_option.unwrap_or_else(HashMap::new);
        let properties_count = properties.len() as c_int;

        let properties_keys: Vec<*const c_char> = properties.keys()
                .map(|&k| to_c_str(k))
                .collect();

        let properties_values: Vec<*const c_char> = properties.values()
                .map(|&v| to_c_str(v))
                .collect();

        let properties_keys_ref = &properties_keys[0] as *const *const c_char;
        let properties_values_ref = &properties_values[0] as *const *const c_char;

        let coreclr_library = libl::Library::new("/usr/local/share/dotnet/shared/Microsoft.NETCore.App/2.0.0")?;
        
        unsafe {
            let coreclr_initialize_fn: libl::Symbol<CoreClrInitializeFn> = coreclr_library.get(b"coreclr_initialize")?;

            assert_eq!(
                coreclr_initialize_fn(
                    exe_path_raw,
                    app_domain_friendly_name_raw,
                    properties_count,
                    properties_keys_ref,
                    properties_values_ref,
                    host_handle_ref,
                    domain_id_ref), 0);
        }

        Ok(CoreClr {
            host_handle: host_handle,
            domain_id: domain_id
        })
    }

    pub fn shutdown(self: Self) -> libl::Result<()> {
        let coreclr_library = libl::Library::new("/usr/local/share/dotnet/shared/Microsoft.NETCore.App/2.0.0")?;

        unsafe {
            let coreclr_shutdown_fn: libl::Symbol<CoreClrShutdownFn> = coreclr_library.get(b"coreclr_shutdown")?;
            assert_eq!(coreclr_shutdown_fn(self.host_handle, self.domain_id), 0)
        }

        Ok(())
    }
}