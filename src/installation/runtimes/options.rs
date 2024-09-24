use inquire::Select;
use crate::main_options;

pub fn choose_runtime(){
    let options = vec![
        "NodeJS",
        "Nginx",
        "Back"
    ];

    let selected_option = Select::new("Select runtime installation:", options.clone())
        .with_vim_mode(false)
        .prompt();

    match selected_option {
        Ok(selection) => {
            let index = options.iter().position(|&x| x == selection).unwrap();

            match index {
                0 => println!("Welcome to NodeJS installation"),
                1 => println!("Welcome to Nginx installation"),
                2 => main_options(),
                _ => println!("Invalid selection"),
            }
        }
        Err(_) => println!("User aborted the selection."),
    }
}