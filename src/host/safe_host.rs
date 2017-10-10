use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint, c_void};

use super::unsafe_host::{coreclr_initialize, coreclr_shutdown};

fn to_c_str<T: Into<Vec<u8>>>(t: T) -> *const c_char {
    CString::new(t).unwrap().as_ptr() as *const c_char
}

pub struct CoreClr {
    host_handle: *const c_void,
    domain_id: c_uint
}

impl CoreClr {
    pub fn init(
        exe_path: &str,
        app_domain_friendly_name: &str,
        properties: HashMap<&str, &str>) -> Self
    {
        let host_handle = 0 as *const c_void;
        let host_handle_ref = &host_handle as *const *const c_void;
        
        let domain_id = 0 as c_uint;
        let domain_id_ref = &domain_id as *const c_uint;

        let exe_path_raw = to_c_str(exe_path);
        let app_domain_friendly_name_raw = to_c_str(app_domain_friendly_name);
        
        let properties_count = properties.len() as c_int;

        let properties_keys: Vec<*const c_char>
            = properties.keys()
                .map(|&k| to_c_str(k))
                .collect();
        
        let properties_values: Vec<*const c_char>
            = properties.values()
                .map(|&v| to_c_str(v))
                .collect();

        let properties_keys_ref = &properties_keys[0] as *const *const c_char;
        let properties_values_ref = &properties_values[0] as *const *const c_char;

        unsafe {
            coreclr_initialize(
                exe_path_raw,
                app_domain_friendly_name_raw,
                properties_count,
                properties_keys_ref,
                properties_values_ref,
                host_handle_ref, domain_id_ref);
        }

        return CoreClr {
            host_handle: host_handle,
            domain_id: domain_id
        }
    }

    pub fn shutdown(self: Self) {
        unsafe {
            coreclr_shutdown(
                self.host_handle,
                self.domain_id);
        }
    }
}