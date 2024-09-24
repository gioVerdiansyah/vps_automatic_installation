use inquire::Select;
use crate::main_options;

pub fn choose_package_manager() {
    let options = vec![
        "pip",
        "Composer",
        "NPM",
        "Back"
    ];

    let selected_option = Select::new("Select package manager installation:", options.clone())
        .with_vim_mode(false)
        .prompt();

    match selected_option {
        Ok(selection) => {
            let index = options.iter().position(|&x| x == selection).unwrap();

            match index {
                0 => println!("Welcome to pip installation"),
                1 => println!("Welcome to Composer installation"),
                2 => println!("Welcome to NPM installation"),
                3 => main_options(),
                _ => println!("Invalid selection"),
            }
        }
        Err(_) => println!("User aborted the selection."),
    }
}