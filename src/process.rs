use std::{
    fs,
    io::{self, Read},
};

use anyhow::{Result, Context};

pub struct CommandFlags {
    pub show_tabs: bool,
    pub number_nonblanks: bool,
    pub show_ends: bool,
    pub numbers: bool    
} 

impl CommandFlags {
    pub fn new() -> Self {
        CommandFlags{
            show_tabs: false,
            number_nonblanks: false,
            show_ends: false,
            numbers: false,
        }
    }
}

pub fn process_input(input: Vec<String>) -> (Vec<String>, Vec<String>) {
    
    let input_iter: Vec<String>  = input.into_iter().collect();
    
    let mut command_vec: Vec<String> = Vec::new();
    let mut file_vec: Vec<String> = Vec::new();

    for element in input_iter {

        let first = element.chars()
            .nth(0)
            .unwrap();

        if first == '-' && element.len() > 1 {
            command_vec.push(element[1..].to_string());
        } else {
            file_vec.push(element);
        }
    }

    (file_vec, command_vec)

}

pub fn collect_files_to_string(file_vec: Vec<String>) -> Result<String> {
    let files = file_vec.into_iter();

    let mut master_string = String::new();

    for file in files {
        let file_contents = fs::read_to_string(&file)
            .with_context(|| format!("Failed to read file {}", &file))?;
        
        master_string.push_str(file_contents.as_str());
    }

    Ok(master_string)
}

pub fn process_commands(command_vec: Vec<String>) -> CommandFlags {
    let mut flags = CommandFlags::new(); 

    for i in command_vec[..] {
        match i.as_mut_str() {
            "T" | "-show-tabs" => flags.show_tabs = true,
            "b" | "-number-nonblanks" => flags.number_nonblanks = true,
            "E" | "-show_ends" => flags.show_ends = true,
            "n" | "-numbers" => flags.numbers = true,
            _ => panic!("invalid option -- \"{}\"", i.as_mut_str),
        }
    }    

    flags
}
