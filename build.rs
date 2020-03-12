use std::{env, path::PathBuf};

fn main() {
  let target = env::var("TARGET").unwrap();
  let out_dir = env::var("OUT_DIR").unwrap();

  // build OpenHardwareMonitorLib.dll so that we can do
  // #using <OpenHardwareMonitorLib.dll>
  let mut msbuild = cc::windows_registry::find(&target, "msbuild.exe").unwrap();

  let current_dir = env::current_dir().unwrap();
  env::set_current_dir(current_dir.join("openhardwaremonitor")).unwrap();

  let mode = if cfg!(debug_assertions) {
    "Debug"
  } else {
    "Release"
  };

  assert!(msbuild
    .arg(format!("/p:Configuration={}", mode))
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
  println!(
    "cargo:rustc-link-search=static=C:\\Program Files (x86)\\Windows \
     Kits\\NETFXSDK\\4.8\\Lib\\um\\x64"
  );

  println!("cargo:rerun-if-changed=src/interface.hpp");

  // The bindgen::Builder is the main entry point
  // to bindgen, and lets you build up options for
  // the resulting bindings.
  let bindings = bindgen::Builder::default()
    .disable_name_namespacing()
    .default_enum_style(bindgen::EnumVariation::Rust {
      non_exhaustive: false,
    })
    // The input header we would like to generate
    // bindings for.
    .header("./src/interface.hpp")
    // Tell cargo to invalidate the built crate whenever any of the
    // included header files changed.
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))

    .no_copy("FFI.*")
    // Finish the builder and generate the bindings.
    .generate()
    // Unwrap the Result and panic on failure.
    .expect("Unable to generate bindings");

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");

  // The user must then put OpenHardwareMonitorLib.dll in
  // the same directory as their executable!
}
