use open_hardware_monitor_sys::*;

#[test]
fn it_works() {
  unsafe {
    let computer = Computer_new();
    assert!(!computer.is_null());

    println!("{:#?}", computer);

    Computer_Open(&computer);

    let char_ptr = Computer_GetReport(&computer);
    println!("{}", char_ptr.to_string().unwrap());
  }
}
