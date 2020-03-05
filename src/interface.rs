use std::os::raw::{c_char, c_float, c_void};

extern "C" {
  /// must call Computer_delete later
  pub fn Computer_new() -> *mut c_void;
  pub fn Computer_delete(computerPtr: *mut c_void);

  /// must call char_ptr_delete later
  pub fn Computer_GetReport(computerPtr: *mut c_void) -> *mut c_char;

  pub fn char_ptr_delete(ptr: *mut c_char);
  pub fn Computer_Open(computerPtr: *mut c_void);
  pub fn Computer_UpdateAll(computerPtr: *mut c_void);
  pub fn Computer_GetValues(computerPtr: *mut c_void) -> *const c_float;
}
