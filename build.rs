use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn run_command(name: &str, args: &[&str], dir: Option<&Path>) {
    println!("Running: {} {}", name, args.join(" "));

    let mut cmd = Command::new(name);
    cmd.args(args);

    if let Some(path) = dir {
        cmd.current_dir(path);
    }

    let status = cmd
        .status()
        .unwrap_or_else(|e| panic!("Failed to execute {}: {}", name, e));

    if !status.success() {
        panic!("{} failed with exit code: {}", name, status);
    }
}

fn find_rlib_file_in_dir(dir: &Path, starts_with: &str, ends_with: &str) -> Option<PathBuf> {
    if !dir.exists() {
        return None;
    }

    println!("Searching in directory: {}", dir.display());

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let filename = entry.file_name().to_string_lossy().to_string();
                    println!("Found file: {}", filename);
                    if filename.starts_with(starts_with) && filename.ends_with(ends_with) {
                        return Some(entry.path());
                    }
                }
            }
        }
        Err(e) => {
            println!("Error reading directory {}: {}", dir.display(), e);
        }
    }

    None
}

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=script.ld");
    println!("cargo:rerun-if-changed=elf.s");
    println!("cargo:rerun-if-changed=build.rs");

    // Check for required tools
    for tool in &["ar", "ld", "objcopy", "nasm", "nm"] {
        let result = Command::new(tool).arg("--version").output();

        if result.is_err() {
            panic!("Can't find {}, needed to build", tool);
        }
    }

    // Get environment variables
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("OUT_DIR: {}", out_dir);

    // Create a working directory
    let work_dir = Path::new(&out_dir).join("tinyrust_build");
    fs::create_dir_all(&work_dir).unwrap();
    println!("Work directory: {}", work_dir.display());

    // First, look for built rlib in the expected locations
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_dir = Path::new(&manifest_dir).join("target");

    println!(
        "Looking for rlib in target directory: {}",
        target_dir.display()
    );

    // Places to look for the rlib
    let mut rlib_file: Option<PathBuf> = None;

    // First check in deps directory
    let deps_dir = target_dir.join(&profile).join("deps");
    rlib_file = find_rlib_file_in_dir(&deps_dir, "libtinyrust-", ".rlib");

    // Copy files to work directory
    let script_ld = include_str!("script.ld");
    let elf_s = include_str!("elf.s");

    let script_path = work_dir.join("script.ld");
    let elf_path = work_dir.join("elf.s");

    fs::write(&script_path, script_ld).unwrap();
    fs::write(&elf_path, elf_s).unwrap();

    // If not found, compile it directly using rustc
    if rlib_file.is_none() {
        println!("Couldn't find rlib, using direct approach");

        println!("Compiling directly from source");

        // Use the direct approach from the original build.sh script
        let rust_src = Path::new(&manifest_dir).join("src").join("lib.rs");
        let lib_tinyrust = work_dir.join("libtinyrust.rlib");

        // Compile using rustc directly
        run_command(
            "rustc",
            &[
                &rust_src.to_string_lossy(),
                "-O",
                "-C",
                "relocation-model=static",
                "--extern",
                "sc=syscall.rs/target/release/libsc.rlib",
                "-o",
                &lib_tinyrust.to_string_lossy(),
            ],
            None,
        );

        rlib_file = Some(lib_tinyrust);
    }

    let rlib_file = rlib_file.expect("Could not find or create tinyrust rlib file");
    println!("Found rlib at: {}", rlib_file.display());

    // Extract objects from rlib
    run_command("ar", &["x", &rlib_file.to_string_lossy()], Some(&work_dir));

    // Find the object file
    let entries = fs::read_dir(&work_dir).unwrap();
    let mut obj_file = None;
    for entry in entries {
        let entry = entry.unwrap();
        let filename = entry.file_name().into_string().unwrap();
        if filename.contains("tinyrust") && filename.ends_with(".o") {
            obj_file = Some(filename);
            break;
        }
    }

    let obj_file = obj_file.expect("Could not find tinyrust object file");
    let tinyrust_o = work_dir.join("tinyrust.o");
    fs::rename(work_dir.join(&obj_file), &tinyrust_o).unwrap();

    // Run objdump for debug info
    let _ = Command::new("objdump")
        .args(&["-dr", &tinyrust_o.to_string_lossy()])
        .status();

    // Link with script
    let payload_path = work_dir.join("payload");
    run_command(
        "ld",
        &[
            "--gc-sections",
            "-e",
            "main",
            "-T",
            &script_path.to_string_lossy(),
            "-o",
            &payload_path.to_string_lossy(),
            &tinyrust_o.to_string_lossy(),
        ],
        None,
    );

    // Extract binary section
    let payload_bin_path = work_dir.join("payload.bin");
    run_command(
        "objcopy",
        &[
            "-j",
            "combined",
            "-O",
            "binary",
            &payload_path.to_string_lossy(),
            &payload_bin_path.to_string_lossy(),
        ],
        None,
    );

    // Get entry point address
    let output = Command::new("nm")
        .args(&["-f", "posix", &payload_path.to_string_lossy()])
        .output()
        .expect("Failed to run nm");

    let nm_output = String::from_utf8_lossy(&output.stdout);

    let entry_line = nm_output
        .lines()
        .find(|line| line.starts_with("main "))
        .expect("Could not find main function in nm output");

    let entry_addr = entry_line.split_whitespace().nth(2).unwrap();
    println!("Entry point address: 0x{}", entry_addr);

    // Assemble the final executable
    let tinyrust_exec = work_dir.join("tinyrust");
    run_command(
        "nasm",
        &[
            "-f",
            "bin",
            "-o",
            &tinyrust_exec.to_string_lossy(),
            &format!("-D entry=0x{}", entry_addr),
            &format!(
                "-D PAYLOAD_PATH=\"{}\"",
                &payload_bin_path.to_string_lossy()
            ),
            &elf_path.to_string_lossy(),
        ],
        None,
    );

    // Make executable and copy to output
    let output_path = Path::new(&manifest_dir)
        .join("target")
        .join(&profile)
        .join("tinyrust");
    fs::copy(&tinyrust_exec, &output_path).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&output_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&output_path, perms).unwrap();
    }

    // Display file info
    println!("Executable created at: {}", output_path.display());

    // Show file size
    println!("File size:");
    let _ = Command::new("wc")
        .args(&["-c", &output_path.to_string_lossy()])
        .status();
}
