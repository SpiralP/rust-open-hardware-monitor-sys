#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use failure::{Error, Fail};
use std::{ffi::c_void, mem, ops::Deref, string};
use widestring::WideCStr;

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

#[derive(Debug, Fail)]
pub enum FFICharPtrError {
  #[fail(display = "pointer is null")]
  Null,

  #[fail(display = "utf16 error: {}", error)]
  Utf16 { error: string::FromUtf16Error },
}

impl FFICharPtr {
  pub fn to_string(&self) -> Result<String, Error> {
    if self.is_null() {
      return Err(FFICharPtrError::Null.into());
    }

    let wide_c_str = unsafe { WideCStr::from_ptr_str(self.ptr) };

    Ok(wide_c_str.to_string()?)
  }
}

impl Deref for FFICharPtr {
  type Target = *mut u16;

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
