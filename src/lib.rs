mod interface;

pub use crate::interface::*;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    unsafe {
      let computer_ptr = Computer_new();
      assert!(!computer_ptr.is_null());

      println!("{:#?}", computer_ptr);

      Computer_Open(computer_ptr);

      let c_str = Computer_GetReport(computer_ptr);
      println!("{}", std::ffi::CStr::from_ptr(c_str).to_string_lossy());

      char_ptr_delete(c_str);

      let values = std::slice::from_raw_parts(Computer_GetValues(computer_ptr), 10);
      println!("{:#?}", values);

      Computer_delete(computer_ptr);
    }
  }
}

pub struct Computer {}
