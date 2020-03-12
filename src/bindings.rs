#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use crate::helpers::take_zeroed;
use failure::{Error, Fail};
use std::{ffi::c_void, ops::Deref, string};
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
      unsafe {
        GCHandle_Free(take_zeroed(self));
      }
    }
  }
}

#[derive(Debug, Fail)]
pub enum FFIWCharPtrError {
  #[fail(display = "pointer is null")]
  Null,

  #[fail(display = "utf16 error: {}", error)]
  Utf16 { error: string::FromUtf16Error },
}

impl FFIWCharPtr {
  pub fn to_string(&self) -> Result<String, Error> {
    if self.is_null() {
      return Err(FFIWCharPtrError::Null.into());
    }

    let wide_c_str = unsafe { WideCStr::from_ptr_str(self.ptr) };

    Ok(wide_c_str.to_string()?)
  }
}

impl Deref for FFIWCharPtr {
  type Target = *mut u16;

  fn deref(&self) -> &Self::Target {
    &self.ptr
  }
}

impl Drop for FFIWCharPtr {
  fn drop(&mut self) {
    if !self.is_null() {
      unsafe {
        WCharPtr_delete(take_zeroed(self));
      }
    }
  }
}
