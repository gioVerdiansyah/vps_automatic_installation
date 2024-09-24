use inquire::Select;
use crate::handling::ascii_text::print_log;
use crate::main_options;

pub fn test_example_project(){
    print_log("info", "This Test Example Project aims to test the installation results whether it runs normally.");
    let options = vec![
        "PHP",
        "NodeJS",
        "Python",
        "Spring Boot",
        "Back"
    ];

    let selected_option = Select::new("Select runtime installation:", options.clone())
        .with_vim_mode(false)
        .prompt();

    match selected_option {
        Ok(selection) => {
            let index = options.iter().position(|&x| x == selection).unwrap();

            match index {
                0 => println!("Welcome to PHP test project"),
                1 => println!("Welcome to NodeJS test project"),
                2 => println!("Welcome to Python test project"),
                3 => println!("Welcome to Spring Boot test project"),
                4 => main_options(),
                _ => println!("Invalid selection"),
            }
        }
        Err(_) => println!("User aborted the selection."),
    }
}