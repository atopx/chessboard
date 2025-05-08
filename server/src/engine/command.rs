use std::process::Child;
use std::process::Command;
use std::process::Stdio;

#[cfg(target_os = "windows")]
pub fn new(libs: &std::path::Path) -> Child {
    use std::os::windows::process::CommandExt;
    Command::new(libs.join("pikafish-windows.exe"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .creation_flags(0x08000000)
        .spawn()
        .expect("Unable to run engine")
}

#[cfg(target_os = "macos")]
pub fn new(libs: &std::path::Path) -> Child {
    Command::new(libs.join("pikafish-macos"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Unable to run engine")
}

#[cfg(target_os = "linux")]
pub fn new(libs: &std::path::Path) -> Child {
    Command::new(libs.join("pikafish-linux"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Unable to run engine")
}
