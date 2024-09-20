use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let dll_filename = "kyber.dll"; // The DLL file is in the root directory

    // Get the build output directory
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Define the path to the DLL in the root directory
    let dll_src_path = PathBuf::from(dll_filename);

    // Define where to copy the DLL in the build output directory
    let dll_dest_path = out_dir.join(dll_filename);

    // Check if the DLL file exists before trying to copy
    if !dll_src_path.exists() {
        panic!("DLL file not found at {}", dll_src_path.display());
    }

    // Copy the DLL from the root folder to the build output directory
    fs::copy(&dll_src_path, &dll_dest_path).expect(&format!(
        "Failed to copy {} to {}",
        dll_src_path.display(),
        dll_dest_path.display()
    ));

    // Tell Cargo to link to the DLL in the output directory
    println!("cargo:rustc-link-search=native={}", out_dir.display());

    // Print information for debugging
    println!("cargo:warning=Copied DLL to: {}", dll_dest_path.display());

    // Make sure the build script reruns if the DLL or the build script changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", dll_src_path.display());
}
