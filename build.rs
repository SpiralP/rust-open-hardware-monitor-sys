use std::env;
use std::path::*;

fn main() {
  let target = env::var("TARGET").unwrap();
  let out_dir = env::var("OUT_DIR").unwrap();

  // build OpenHardwareMonitorLib.dll so that we can do
  // #using <OpenHardwareMonitorLib.dll>
  let mut msbuild = cc::windows_registry::find(&target, "msbuild.exe").unwrap();

  let current_dir = env::current_dir().unwrap();
  env::set_current_dir(current_dir.join("openhardwaremonitor")).unwrap();

  assert!(msbuild
    .arg("/p:Configuration=Release")
    .arg(format!("/p:OutputPath={}", out_dir))
    .arg("OpenHardwareMonitorLib.csproj")
    .status()
    .unwrap()
    .success());

  env::set_current_dir(current_dir).unwrap();

  env::set_var("LIBPATH", out_dir);

  println!("cargo:rerun-if-changed=src/interface.cpp");

  cc::Build::new()
    .file("src/interface.cpp")
    .flag("/clr")
    .compile("interface");

  // MSCOREE.lib
  println!("cargo:rustc-link-search=static=C:\\Program Files (x86)\\Windows Kits\\NETFXSDK\\4.8\\Lib\\um\\x64");

  // The user must then put OpenHardwareMonitorLib.dll in
  // the same directory as their executable!
}
