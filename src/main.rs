use dnm::run_cli;
#[cfg(target_os = "windows")]
use dnm::is_gui_console;

fn main() {
    let exit_code = run_cli();

    #[cfg(target_os = "windows")]
    if is_gui_console() {
        pause();
    }

    std::process::exit(exit_code);
}

#[cfg(target_os = "windows")]
fn pause() {
    use std::io::{self, Write};
    println!();
    print!("\n按 Enter 键退出...");
    let _ = io::stdout().flush();
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
}
