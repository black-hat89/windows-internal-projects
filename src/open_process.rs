use std::io;
use std::mem::zeroed;
use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32, Process32First, Process32Next, TH32CS_SNAPPROCESS,
};

pub fn get_open_process() -> Result<(), Box<dyn std::error::Error>> {
    // Ask Windows to create a snapshot of all running processes.
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };

    // Check whether Windows failed to create the snapshot.
    if snapshot == INVALID_HANDLE_VALUE {
        return Err(io::Error::last_os_error().into());
    }

    unsafe {
        //  Create an empty process information container.
        let mut process_entry: PROCESSENTRY32 = zeroed();
        // Tell Windows the size of our container.
        process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
        println!("Struct size: {} bytes", process_entry.dwSize);
        let result = Process32First(snapshot, &mut process_entry);
        // Check whether Windows found the first process.
        if result == 0 {
            return Err(io::Error::last_os_error().into());
        }

        let name_length = process_entry
            .szExeFile
            .iter()
            .position(|&character| character == 0)
            .unwrap_or(process_entry.szExeFile.len());

        let process_name: String = process_entry.szExeFile[..name_length]
            .iter()
            .map(|&c| c as u8)
            .map(char::from)
            .collect();

        println!(
            "PID: {} | Name: {}",
            process_entry.th32ProcessID, process_name
        );

        // Step 6: Get and print all remaining processes.
        while Process32Next(snapshot, &mut process_entry) != 0 {
            let name_length = process_entry
                .szExeFile
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(process_entry.szExeFile.len());

            let process_name: String = process_entry.szExeFile[..name_length]
                .iter()
                .map(|&c| c as u8)
                .map(char::from)
                .collect();

            println!(
                "PID: {} | Name: {}",
                process_entry.th32ProcessID, process_name
            );
        }
    }

    unsafe {
        CloseHandle(snapshot);
    }

    println!("Snapshot handle: {:?}", snapshot);

    Ok(())
}
