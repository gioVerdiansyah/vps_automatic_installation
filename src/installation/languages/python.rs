use std::process::{Command, exit};
use dialoguer::{Confirm, Input};
use crate::handling::ascii_text::{print_process, print_log};

pub fn install_python() {
    print_log("warning", "This will install the latest version of Python !");
    print_log("default", "Enter N to enter your python version.");
    let confirm = Confirm::new()
        .with_prompt("Do you want to continue?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        install(None)
    } else {
        let py_version: String = Input::new()
            .with_prompt("Enter Python version :")
            .interact_text()
            .unwrap();
        install(Some(&py_version))
    }
}

fn install(version: Option<&str>){
    let ver: &str = version.unwrap_or("3");
    print_process(&format!("Installing Python{}...", ver));

    print_log("info", "Updating system...");
    let update_status = Command::new("sudo")
        .arg("apt-get")
        .arg("update")
        .arg("-y")
        .status()
        .expect("Failed to execute apt-get update");

    if update_status.success() {
        print_log("info", "System updated!");

        let install_status = Command::new("sudo")
            .arg("apt-get")
            .arg("install")
            .arg(&format!("python{}",ver))
            .arg("-y")
            .status()
            .expect("Failed to execute apt-get install python3");

        if install_status.success() {
            print_log("success", "Python3 installed successfully.");
        } else {
            print_log("error", &format!("Failed to install Python{}", ver));
            exit(1)
        }
    } else {
        print_log("error", "Failed to update the system.");
        exit(1)
    }
}