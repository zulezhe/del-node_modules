use std::process::{Command, Stdio};

/// Check if the current process is running with elevated (admin/root) privileges
pub fn is_elevated() -> bool {
    #[cfg(target_os = "windows")]
    {
        is_elevated_windows()
    }

    #[cfg(not(target_os = "windows"))]
    {
        unsafe { libc::geteuid() == 0 }
    }
}

#[cfg(target_os = "windows")]
fn is_elevated_windows() -> bool {
    // Try to open a privileged token to check elevation status
    use std::mem;

    type BOOL = i32;
    type HANDLE = isize;
    type DWORD = u32;

    const TOKEN_QUERY: DWORD = 0x0008;
    const TOKEN_ELEVATION_CLASS: DWORD = 20;

    #[repr(C)]
    struct TOKEN_ELEVATION {
        token_is_elevated: DWORD,
    }

    extern "system" {
        fn OpenProcessToken(process_handle: HANDLE, desired_access: DWORD, token_handle: *mut HANDLE) -> BOOL;
        fn GetTokenInformation(token_handle: HANDLE, token_information_class: DWORD, token_information: *mut u8, token_information_length: DWORD, return_length: *mut DWORD) -> BOOL;
        fn GetCurrentProcess() -> HANDLE;
        fn CloseHandle(handle: HANDLE) -> BOOL;
    }

    unsafe {
        let current_process = GetCurrentProcess();
        let mut token_handle: HANDLE = 0;

        if OpenProcessToken(current_process, TOKEN_QUERY, &mut token_handle) == 0 {
            return false;
        }

        let mut elevation = mem::zeroed::<TOKEN_ELEVATION>();
        let mut returned_length: DWORD = 0;

        let result = GetTokenInformation(
            token_handle,
            TOKEN_ELEVATION_CLASS,
            &mut elevation as *mut _ as *mut u8,
            mem::size_of::<TOKEN_ELEVATION>() as DWORD,
            &mut returned_length,
        );

        CloseHandle(token_handle);

        result != 0 && elevation.token_is_elevated != 0
    }
}

/// Re-launch the current program with elevated privileges
pub fn elevate(args: &[String]) -> Result<(), String> {
    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Cannot get current executable path: {}", e))?;

    #[cfg(target_os = "windows")]
    {
        elevate_windows(&current_exe, args)
    }

    #[cfg(not(target_os = "windows"))]
    {
        elevate_unix(&current_exe, args)
    }
}

#[cfg(target_os = "windows")]
fn elevate_windows(exe_path: &std::path::Path, args: &[String]) -> Result<(), String> {
    // Use PowerShell's Start-Process -Verb RunAs to trigger UAC elevation
    let args_str = args.iter()
        .map(|a| if a.contains(' ') { format!("\"{}\"", a) } else { a.clone() })
        .collect::<Vec<_>>()
        .join(" ");

    let ps_command = format!(
        "Start-Process -FilePath '{}' -ArgumentList '{}' -Verb RunAs",
        exe_path.display().to_string().replace("'", "''"),
        args_str.replace("'", "''"),
    );

    let output = Command::new("powershell")
        .args(["-Command", &ps_command])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map_err(|e| format!("Failed to launch PowerShell: {}", e))?;

    if output.status.success() {
        // The elevated process runs in a new window, we should exit the current one
        std::process::exit(0);
    } else {
        Err("UAC elevation was cancelled or failed".to_string())
    }
}

#[cfg(not(target_os = "windows"))]
fn elevate_unix(exe_path: &std::path::Path, args: &[String]) -> Result<(), String> {
    let status = Command::new("sudo")
        .arg(exe_path)
        .args(args)
        .status()
        .map_err(|e| format!("Failed to run with sudo: {}", e))?;

    if status.success() {
        std::process::exit(0);
    } else {
        Err("sudo execution failed or was cancelled".to_string())
    }
}
