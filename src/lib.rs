mod bindings;

pub use crate::bindings::*;
use std::mem;

pub struct Computer {
  inner: FFIGCHandle,
}
impl Computer {
  pub fn new() -> Result<Self, ()> {
    let inner = unsafe { Computer_new() };
    if inner.is_null() {
      return Err(());
    }

    Ok(Self { inner })
  }

  pub fn open() -> Result<Self, ()> {
    let computer = Self::new()?;

    unsafe {
      Computer_Open(&computer.inner);
    }

    Ok(computer)
  }

  pub fn get_report(&self) -> String {
    let char_ptr = unsafe { Computer_GetReport(&self.inner) };

    char_ptr.to_string()
  }

  pub fn get_hardwares(&self) -> Vec<Hardware> {
    let mut ffi_hardwares = unsafe { Computer_GetHardwares(&self.inner) };

    ffi_hardwares.items[..ffi_hardwares.length as usize]
      .iter_mut()
      .map(|mut ffi_hardware| {
        let mut swap_me: FFIHardware = unsafe { mem::zeroed() };
        mem::swap(&mut swap_me, &mut ffi_hardware);

        Hardware::from_ffi(swap_me)
      })
      .collect()
  }
}

#[derive(Debug)]
pub struct Hardware {
  inner: FFIGCHandle,
  name: String,
  hardware_type: FFIHardwareType,
}
impl Hardware {
  fn from_ffi(ffi_hardware: FFIHardware) -> Self {
    Self {
      inner: ffi_hardware.ptr,
      name: ffi_hardware.name.to_string(),
      hardware_type: ffi_hardware.hardwareType,
    }
  }

  pub fn update(&self) {
    unsafe {
      Hardware_Update(&self.inner);
    }
  }

  pub fn get_sub_hardwares(&self) -> Vec<Hardware> {
    let mut ffi_hardwares = unsafe { Hardware_GetSubHardwares(&self.inner) };

    ffi_hardwares.items[..ffi_hardwares.length as usize]
      .iter_mut()
      .map(|mut ffi_hardware| {
        let mut swap_me: FFIHardware = unsafe { mem::zeroed() };
        mem::swap(&mut swap_me, &mut ffi_hardware);

        Hardware::from_ffi(swap_me)
      })
      .collect()
  }

  pub fn get_sensors(&self) -> Vec<Sensor> {
    let mut ffi_sensors = unsafe { Hardware_GetSensors(&self.inner) };

    ffi_sensors.items[..ffi_sensors.length as usize]
      .iter_mut()
      .map(|mut ffi_sensor| {
        let mut swap_me: FFISensor = unsafe { mem::zeroed() };
        mem::swap(&mut swap_me, &mut ffi_sensor);

        Sensor::from_ffi(swap_me)
      })
      .collect()
  }
}

#[derive(Debug)]
pub struct Sensor {
  inner: FFIGCHandle,
  name: String,
  sensor_type: FFISensorType,
}
impl Sensor {
  fn from_ffi(ffi_sensor: FFISensor) -> Self {
    Self {
      inner: ffi_sensor.ptr,
      name: ffi_sensor.name.to_string(),
      sensor_type: ffi_sensor.sensorType,
    }
  }

  pub fn get_value(&self) -> f32 {
    unsafe { Sensor_GetValue(&self.inner) }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_computer() {
    let computer = Computer::open().unwrap();
    println!("{}", computer.get_report());

    let hardwares = computer.get_hardwares();
    println!("Hardwares: {:#?}", hardwares);

    let hardware = &hardwares[0];
    hardware.update();

    println!("SubHardwares: {:#?}", hardware.get_sub_hardwares());

    let sensors = hardware.get_sensors();
    println!("Sensors: {:#?}", sensors);

    for sensor in sensors {
      if sensor.sensor_type == FFISensorType::Temperature {
        println!("{:?} {}", sensor, sensor.get_value());
      }
    }
  }

  #[test]
  fn it_works() {
    unsafe {
      let computer = Computer_new();
      assert!(!computer.is_null());

      println!("{:#?}", computer);

      Computer_Open(&computer);

      let char_ptr = Computer_GetReport(&computer);
      println!("{}", char_ptr.to_string());
    }
  }
}
