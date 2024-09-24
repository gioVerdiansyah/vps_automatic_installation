use chrono::Local;
use terminal_size::{Width, terminal_size};
use colored::*;

pub fn print_welcome() {
    println!(
        "{}", r"                                        __     ______  ____
                                        \ \   / /  _ \/ ___|
                                         \ \ / /| |_) \___ \
                                          \ V / |  __/ ___) |
    _         _                        _   \_/  |_| _ |____/    _        _ _       _   _
   / \  _   _| |_ ___  _ __ ___   __ _| |_(_) ___  (_)_ __  ___| |_ __ _| | | __ _| |_(_) ___  _ __
  / _ \| | | | __/ _ \| '_ ` _ \ / _` | __| |/ __| | | '_ \/ __| __/ _` | | |/ _` | __| |/ _ \| '_ \
 / ___ \ |_| | || (_) | | | | | | (_| | |_| | (__  | | | | \__ \ || (_| | | | (_| | |_| | (_) | | | |
/_/   \_\__,_|\__\___/|_| |_| |_|\__,_|\__|_|\___| |_|_| |_|___/\__\__,_|_|_|\__,_|\__|_|\___/|_| |_|
Created by: VerdIXI                                         Github: https://github.com/gioVerdiansyah".truecolor(19, 161, 14)
    )
}

pub fn print_banner(message: &str) {
    let width = terminal_size().map(|(Width(w), _)| w).unwrap_or(80);
    let text = format!(" {} ", message);
    let text_length = text.len() as u16;
    let total_length = width as u16;
    let padding = (total_length - text_length) / 2;

    // println!("{}", "#".repeat(total_length as usize));

    let right_padding_length = total_length - text_length - padding;
    let left_padding = "#".repeat(padding as usize);
    let right_padding = "#".repeat(right_padding_length as usize);

    println!("{}{}{}", left_padding.truecolor(19, 161, 14), text.truecolor(19, 161, 14), right_padding.truecolor(19, 161, 14));

    // println!("{}", "#".repeat(total_length as usize));
}

pub fn print_process(message: &str) {
    let width = terminal_size().map(|(Width(w), _)| w).unwrap_or(80);
    let text = format!(" {} ", message);
    let text_length = text.len() as u16;
    let total_length = width as u16;
    let padding = (total_length - text_length) / 2;

    let left_padding = " ".repeat((padding as usize) - 10);

    println!("{}{}{}{}", left_padding.truecolor(19, 161, 14), "==========".truecolor(19, 161, 14), text.truecolor(19, 161, 14), "==========".truecolor(19, 161, 14));
}

pub fn print_log(level: &str, message: &str) {
    let current_time = Local::now().format("%H:%M").to_string();
    let (label, colorized_message) = match level {
        "info" => ("INFO", message.bright_cyan().to_string()),
        "warning" => ("WARNING!", message.bright_yellow().to_string()),
        "danger" => ("DANGER!", message.bright_red().to_string()),
        "error" => ("ERROR!", message.bright_red().to_string()),
        "success" => ("SUCCESS!", message.truecolor(19, 161, 14).to_string()),
        "default" => ("", message.to_string()),
        _ => ("", "".to_string()),
    };

    if label.is_empty() {
        println!("[{}] > {}", current_time, colorized_message);
    } else {
        println!(
            "[{}] > {} {}",
            current_time,
            label.bright_white().bold(),
            colorized_message
        );
    }
}