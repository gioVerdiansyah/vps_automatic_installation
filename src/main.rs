mod handling;
pub mod installation;

use std::process::exit;
use inquire::Select;
use nix::unistd::Uid;
use crate::handling::ascii_text::{print_log, print_welcome};
use crate::installation::databases::options::choose_db;
use crate::installation::languages::options::choose_lang;
use crate::installation::package_managers::options::choose_package_manager;
use crate::installation::runtimes::options::choose_runtime;
use crate::installation::test_example_project::options::test_example_project;

fn main() {
    if Uid::effective().is_root() {
        print_welcome();

        main_options()
    }else{
        print_log("danger","You're not run with sudo!");
        print_log("default","Please run with sudo");
        print_log("default","Example: \"sudo automization_script\" or \"sudo ./automization_script\"")
    }
}

fn main_options(){
    let options = vec![
        "Programming Language",
        "Runtime",
        "Database",
        "Package Manager",
        "Test Example Project",
        "Exit"
    ];

    let selected_option = Select::new("Select installation:", options.clone())
        .with_vim_mode(false)
        .prompt();

    match selected_option {
        Ok(selection) => {
            let index = options.iter().position(|&x| x == selection).unwrap();

            match index {
                0 => choose_lang(),
                1 => choose_runtime(),
                2 => choose_db(),
                3 => choose_package_manager(),
                4 => test_example_project(),
                5 => exit(0),
                _ => println!("Invalid selection"),
            }
        },
        Err(_) => println!("User aborted the selection."),
    }
}