use std::process::Command;
use dialoguer::Confirm;
use crate::handling::ascii_text::print_log;

pub fn check_version_and_install<F>(cmd: &str, install: F) where F: Fn(){
    if let Ok(output) = Command::new(cmd).arg("--version").output() {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            print_log("warning",&format!("{} already installed, version: {}", cmd, version));
            let confirm = Confirm::new()
                .with_prompt("Do you want to continue?")
                .default(false)
                .interact()
                .unwrap();

            if confirm {
                install()
            } else {
                print_log("info","Exiting program");
                print_log("default","See you :D")
            }
        } else {
            print_log("error",&format!("Failed to check {} version.", cmd));
        }
    } else {
        install()
    }
}

pub fn check_version(cmd: &str) -> bool{
    if let Ok(output) = Command::new(cmd).arg("--version").output() {
        if output.status.success() {
            true
        } else {
            print_log("error",&format!("Failed to check {} version.", cmd));
            false
        }
    }else{
        false
    }
}