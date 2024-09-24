mod handling;

use inquire::Select;
use crate::handling::ascii_text::print_welcome;
use crate::handling::installation::golang::golang;
use crate::handling::installation::nodejs::nodejs;
use crate::handling::installation::php::php;
use crate::handling::installation::python::python;
use crate::handling::installation::spring_boot::spring_boot;

fn main() {
    print_welcome();

    let options = vec![
        "Python",
        "Golang",
        "NodeJS",
        "PHP",
        "Spring Boot (Java)",
    ];

    let selected_option = Select::new("Select Installation:", options.clone())
        .with_vim_mode(false)
        .prompt();

    match selected_option {
        Ok(selection) => {
            let index = options.iter().position(|&x| x == selection).unwrap();

            match index {
                0 => python(),
                1 => golang(),
                2 => nodejs(),
                3 => php(),
                4 => spring_boot(),
                _ => println!("Invalid selection"),
            }
        },
        Err(_) => println!("User aborted the selection."),
    }
}
