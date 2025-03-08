use dll_syringe::{process::OwnedProcess, Syringe};

fn main() {
    println!("Injecting TAS tool...");
    try_inject();
}

fn get_library_path() -> std::path::PathBuf {
    use std::fs;

    // Contents of the dll to inject
    let lib_inject = include_bytes!(env!("CARGO_CDYLIB_FILE_INJECTED"));

    // Write file to temp dir
    let mut file_path = std::env::temp_dir();
    file_path.push("control_tas.dll");
    let _ = fs::write(file_path.clone(), lib_inject);

    file_path
}

pub fn try_inject() {
    // find target process by name
    let Some(target_process) = OwnedProcess::find_first_by_name("Control") else {
        println!("Failed to find the control process");
        return;
    };

    // create a new syringe for the target process
    let syringe = Syringe::for_process(target_process);

    let library = get_library_path();
    
    if let Err(e) = syringe.inject(library) {
        println!("Failed to inject dll: {e}")
    } else {
        println!("Successfully injected DLL")
    }
}