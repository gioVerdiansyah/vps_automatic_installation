use inquire::Select;
use crate::main_options;

pub fn choose_db(){
    let options = vec![
        "MySQL",
        "MongoDB",
        "Back"
    ];

    let selected_option = Select::new("Select database installation:", options.clone())
        .with_vim_mode(false)
        .prompt();

    match selected_option {
        Ok(selection) => {
            let index = options.iter().position(|&x| x == selection).unwrap();

            match index {
                0 => println!("Welcome to MySQL installation"),
                1 => println!("Welcome to Golang installation"),
                2 => main_options(),
                _ => println!("Invalid selection"),
            }
        }
        Err(_) => println!("User aborted the selection."),
    }
}