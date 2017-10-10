use std::os::raw::{c_char, c_int, c_uint, c_void};

#[link(name = "coreclr")]
extern {
    pub fn coreclr_initialize(
        exe_path: *const c_char,
        app_domain_friendly_name: *const c_char,
        property_count: c_int,
        property_keys: *const *const c_char,
        property_values: *const *const c_char,
        host_handle: *const *const c_void,
        domain_id: *const c_uint) -> c_int;
    
    pub fn coreclr_shutdown(
        host_handle: *const c_void,
        domain_id: c_uint) -> c_int;
    
    pub fn coreclr_shutdown_2(
        host_handle: *const c_void,
        domain_id: c_uint,
        latched_exit_code: *const c_int) -> c_int;
    
    pub fn coreclr_create_delegate(
        host_handle: *const c_void,
        domain_id: c_uint,
        entry_point_assembly_name: *const c_char,
        entry_point_type_name: *const c_char,
        entry_point_method_name: *const c_char,
        delegate: *const *const c_void) -> c_int;
    
    pub fn coreclr_execute_assembly(
        host_handle: *const c_void,
        domain_id: c_uint,
        argc: c_int,
        argv: *const *const c_char,
        managed_assembly_path: *const c_char,
        exit_code: *const c_uint) -> c_int;
}