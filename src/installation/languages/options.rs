use inquire::Select;
use crate::installation::languages::python::install_python;
use crate::main_options;

pub fn choose_lang() {
    let options = vec![
        "Python",
        "Golang",
        "PHP",
        "Java",
        "Back"
    ];

    let selected_option = Select::new("Select language installation:", options.clone())
        .with_vim_mode(false)
        .prompt();

    match selected_option {
        Ok(selection) => {
            let index = options.iter().position(|&x| x == selection).unwrap();

            match index {
                0 => install_python(),
                1 => println!("Welcome to Golang installation"),
                2 => println!("Welcome to PHP installation"),
                3 => println!("Welcome to Java installation"),
                4 => main_options(),
                _ => println!("Invalid selection"),
            }
        }
        Err(_) => println!("User aborted the selection."),
    }
}