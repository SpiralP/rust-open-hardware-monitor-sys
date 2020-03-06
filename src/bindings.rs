#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::{
  ffi::{c_void, CStr},
  mem,
  ops::Deref,
  os::raw::c_char,
};

impl Deref for FFIGCHandle {
  type Target = *mut c_void;

  fn deref(&self) -> &Self::Target {
    &self.ptr
  }
}

impl Drop for FFIGCHandle {
  fn drop(&mut self) {
    if !self.is_null() {
      // println!("dropping GCHandle");
      unsafe {
        // hack to allow us to "take" and not use a reference
        let mut swap_me: FFIGCHandle = mem::zeroed();
        mem::swap(&mut swap_me, self);
        GCHandle_Free(swap_me);
      }
    }
  }
}

impl ToString for FFICharPtr {
  fn to_string(&self) -> String {
    let c_str = unsafe { CStr::from_ptr(self.ptr) };
    c_str.to_string_lossy().to_string()
  }
}

impl Deref for FFICharPtr {
  type Target = *mut c_char;

  fn deref(&self) -> &Self::Target {
    &self.ptr
  }
}

impl Drop for FFICharPtr {
  fn drop(&mut self) {
    if !self.is_null() {
      // println!("dropping CharPtr");
      unsafe {
        // hack to allow us to "take" and not use a reference
        let mut swap_me: FFICharPtr = mem::zeroed();
        mem::swap(&mut swap_me, self);
        CharPtr_delete(swap_me);
      }
    }
  }
}
