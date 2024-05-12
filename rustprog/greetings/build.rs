use std::env;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;

// This is the main function that will be executed when the build script is run.
fn main() {
    // Get the current working directory
    // OUT_DIR is a directory path that cargo has set aside for build scripts to place their output.
    let out_dir = env::var("OUT_DIR").unwrap();

    // Build the paths for the source and destination files
    // The source executable path is constructed by navigating up three directories from OUT_DIR and appending "greetings".
    let src_exe_path = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("greetings");
    // The destination executable path is constructed by appending "bin/greetings" to the HOME directory.
    let dst_exe_path = Path::new(&env::var("HOME").unwrap())
        .join("bin")
        .join("greetings");

    // Build the paths for the source and destination files
    // The source text file path is constructed by appending "src/pithy.txt" to the CARGO_MANIFEST_DIR directory.
    let src_txt_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src")
        .join("pithy.txt");
    // The destination text file path is constructed by appending "bin/pithy.txt" to the HOME directory.
    let dst_txt_path = Path::new(&env::var("HOME").unwrap())
        .join("bin")
        .join("pithy.txt");

    // Check if the cargo clean command is being run
    // If CARGO_CLEAN is set to "1", it means the cargo clean command is being run.
    if env::var("CARGO_CLEAN").unwrap_or_default() == "1" {
        // Remove the symbolic links
        // If the symbolic links cannot be removed, an error message is printed to the standard error.
        fs::remove_file(&dst_exe_path).unwrap_or_else(|err| {
            eprintln!("Failed to remove symlink {:?}: {}", dst_exe_path, err)
        });
        fs::remove_file(&dst_txt_path).unwrap_or_else(|err| {
            eprintln!("Failed to remove symlink {:?}: {}", dst_txt_path, err)
        });
    } else {
        // Remove the old symbolic links
        // If the old symbolic links cannot be removed, the error is ignored.
        let _ = fs::remove_file(&dst_exe_path);
        let _ = fs::remove_file(&dst_txt_path);

        // Create new symbolic links
        // If the new symbolic links cannot be created, the program will panic and print an error message.
        symlink(&src_exe_path, &dst_exe_path).unwrap_or_else(|err| {
            panic!(
                "Failed to create symlink from {:?} to {:?}: {}",
                src_exe_path, dst_exe_path, err
            )
        });
        symlink(&src_txt_path, &dst_txt_path).unwrap_or_else(|err| {
            panic!(
                "Failed to create symlink from {:?} to {:?}: {}",
                src_txt_path, dst_txt_path, err
            )
        });
    }
}