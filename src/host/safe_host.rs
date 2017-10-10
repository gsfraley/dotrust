use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::path::Path;

use super::unsafe_host::{
    coreclr_create_delegate, coreclr_execute_assembly,
    coreclr_initialize,
    coreclr_shutdown, coreclr_shutdown_2};

fn to_c_str<T: Into<Vec<u8>>>(t: T) -> *const c_char {
    CString::new(t).unwrap().as_ptr() as *const c_char
}

pub struct CoreClr {
    host_handle: *const c_void,
    domain_id: c_uint
}

pub struct CoreClrDelegate {
    pub delegate_handle: *const c_void
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
            assert_eq!(
                coreclr_initialize(
                exe_path_raw,
                app_domain_friendly_name_raw,
                properties_count,
                properties_keys_ref,
                properties_values_ref,
                host_handle_ref, domain_id_ref), 0);
        }

        CoreClr {
            host_handle: host_handle,
            domain_id: domain_id
        }
    }

    pub fn shutdown(self: Self) {
        unsafe {
            assert_eq!(
                coreclr_shutdown(
                    self.host_handle,
                    self.domain_id), 0);
        }
    }

    pub fn shutdown_2(self: Self) -> c_int {
        let latched_exit_code = 0 as c_int;
        let latched_exit_code_ref = &latched_exit_code as *const c_int;

        unsafe {
            assert_eq!(
                coreclr_shutdown_2(
                    self.host_handle,
                    self.domain_id,
                    latched_exit_code_ref), 0);
        }

        latched_exit_code
    }
    
    pub fn create_delegate(self: &Self) -> CoreClrDelegate {
        let delegate_handle = 0 as *const c_void;
        let delegate_handle_ref = &delegate_handle as *const *const c_void;

        unsafe {
            assert_eq!(
                coreclr_create_delegate(
                    self.host_handle as *const c_void,
                    self.domain_id as c_uint,
                    0 as *const c_char,
                    0 as *const c_char,
                    0 as *const c_char,
                    delegate_handle_ref), 0);
        }

        CoreClrDelegate {
            delegate_handle: delegate_handle
        }
    }

    pub fn execute_assembly(self: &Self, args: Vec<&str>, managed_assembly_path: &Path) -> c_uint {
        let exit_code = 0 as c_uint;
        let exit_code_ref = &exit_code as *const c_uint;

        let args_count = args.len() as c_int;
        let args_raw: Vec<*const c_char> = args.iter()
            .map(|&a| to_c_str(a))
            .collect();
        
        let args_raw_ref = &args_raw[0] as *const *const c_char;
        let managed_assembly_path: *const c_char = to_c_str(managed_assembly_path.to_str().unwrap());

        unsafe {
            assert_eq!(
                coreclr_execute_assembly(
                    self.host_handle,
                    self.domain_id,
                    args_count,
                    args_raw_ref,
                    managed_assembly_path,
                    exit_code_ref), 0);
        }

        exit_code
    }
}