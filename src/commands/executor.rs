use crate::commands::validator::Command;

use crate::macros::CMD_LIST;
use colored::Colorize;
use std::collections::VecDeque;
use std::env;
use std::path::Path;
use std::{fs, fs::File, io::prelude::*};

impl Command {
    pub fn execute_command(self, command_stack: &VecDeque<String>) -> bool {
        if self.operation.trim() == "" {
            return false;
        } else if self.operation == "cmd" && self.operand_1.as_ref().unwrap() == "--help" {
            cmd_helper();
        } else if self.operation == "mkdir" {
            make_dir(&self.operand_1.unwrap());
        } else if self.operation == "cd" {
            change_dir(self.operand_1.unwrap());
        } else if self.operation == "rmdir" {
            remove_dir(&self.operand_1.unwrap());
        } else if self.operation == "make" {
            create_file(&self.operand_1.unwrap());
        } else if self.operation == "rm" {
            remove_file(&self.operand_1.unwrap());
        } else if self.operation == "view" {
            open_file(&self.operand_1.unwrap());
        } else if self.operation == "edit" {
            todo!();
        } else if self.operation == "pwd" {
            println!("{}", format!("{}", get_current_dir()).italic());
        } else if self.operation == "ls" {
            let path = match self.operand_1 {
                Some(path) => {
                    let current_path = get_current_dir();
                    (current_path.trim().to_owned() + "/" + &path.trim().to_owned() + "/")
                        .to_string()
                }
                None => "./".to_string(),
            };
            for file in fs::read_dir(&path).unwrap() {
                let path = file.unwrap().path().display().to_string();
                let segments: Vec<&str> = path.split("/").collect();
                println!("{}", segments.last().unwrap());
            }
        } else if self.operation == "history" {
            get_cmd_history(command_stack);
        } else {
            println!(
                "Invalid Command `{}`. Type `cmd --help` for command list",
                self.operation
            );
            return false;
        }
        true
    }
}

pub fn cmd_helper() {
    println!("{}", CMD_LIST);
}
fn make_dir(path: &str) {
    fs::create_dir_all(path).expect(&format!(
        "Could not create folder. Incorrect path :{}",
        path
    ));
}

fn change_dir(path: String) {
    let new_dir = Path::new(&path);
    if let Ok(_) = env::set_current_dir(new_dir) {
    } else {
        println!(
            "Could not change current directory. Invalid path: {}",
            new_dir.display()
        );
    }
}

fn remove_dir(path: &str) {
    fs::remove_dir_all(path).expect(&format!(
        "Could not delete folder. Incorrect path :{}",
        path
    ));
}

pub fn get_current_dir() -> String {
    env::current_dir().unwrap().display().to_string()
}

fn create_file(file_name: &str) {
    File::create(file_name).expect(&format!("Could not create file  `{}`", file_name));
}

fn remove_file(file_name: &str) {
    fs::remove_file(file_name).expect(&format!("Could not create file  `{}`", file_name));
}

fn open_file(file_name: &str) {
    let file_contents =
        fs::read_to_string(file_name).expect(&format!("Could not create file  `{}`", file_name));
    println!("{}", file_contents);
}

fn get_cmd_history(cmd_stack: &VecDeque<String>) {
    for cmd in cmd_stack.iter() {
        println!("{}", cmd);
    }
}
