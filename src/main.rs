use colored::Colorize;
use std::collections::VecDeque;
use termion;

mod commands;
mod macros;
use crate::commands::{executor::get_current_dir, keys::read_key, validator::validate_command};

use std::io::{self, stdout, Read, Write};
fn main() {
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    println!("WELCOME");
    println!("CUSTOM IDE V1.0");
    println!();
    let mut command_stack: VecDeque<String> = VecDeque::new();
    loop {
        // TODO : key press up down cmd toggle
        println!("Type `cmd --help` for command list");
        let mut input = String::new();
        print!(
            "{} {} ",
            format!("{}", get_current_dir()).bold().green(),
            format!("==>").bold().green()
        );
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Error getting command");

        let cmd = input.trim();
        if cmd.to_lowercase() == "quit" {
            break;
        }
        if validate_command(cmd.to_string())
            .unwrap()
            .execute_command(&command_stack)
        {
            if command_stack.len() == 1000 {
                command_stack.pop_front();
            }
            if !command_stack.is_empty() {
                if cmd != command_stack.back().unwrap().to_string() {
                    command_stack.push_back(cmd.to_string());
                }
            } else {
                command_stack.push_back(cmd.to_string());
            }
        }
    }
}
