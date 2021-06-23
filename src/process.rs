use std::fs;
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

//fn print_command_flags(flags: CommandFlags) {
//    println!("Show tabs{}
//    Number NonBlanks {}
//    Numbers {}
//    Show Ends {}", flags.show_tabs, flags.number_nonblanks, flags.numbers, flags.show_ends);
//}

pub fn process_commands(command_vec: Vec<String>) -> CommandFlags {
    let mut flags = CommandFlags::new(); 

    for i in command_vec {
        match i.as_str() {
            "T" | "-show-tabs" => flags.show_tabs = true,
            "b" | "-number-nonblanks" => flags.number_nonblanks = true, //this overrides -n
            "E" | "-show_ends" => flags.show_ends = true,
            "n" | "-numbers" => flags.numbers = true,
            "A" | "-show-all" | "ET" => { flags.show_tabs = true; flags.show_ends = true;},
            _ => panic!("invalid option -- \"{}\"", i.as_str()),
        }
    }
    flags
}

fn number_lines(multiline_string: String) -> String {
    let lines: Vec<&str> = multiline_string
        .as_str()
        .split_inclusive("\n")
        .collect();

    let mut output = String::new();

    let mut number: u32 = 1;
    for line in lines {
        let numbered_line = &mut number.to_string();
        numbered_line.push_str(&(format!(" {}", line)));
        number += 1; 
        &output.push_str(&(format!("     {}", numbered_line)));
    }
    
    output
}

fn number_nonblanks(multiline_string: String) -> String {
    let lines: Vec<&str> = multiline_string
        .as_str()
        .split_inclusive("\n")
        .collect();

    let mut output = String::new();

    let mut number: u32 = 1;
    for line in lines {
        if line != "\n" {
            let numbered_line = &mut number.to_string();
            numbered_line.push_str(&(format!(" {}", line)));
            number += 1; 
            &output.push_str(&(format!("     {}", numbered_line)));
        } else {
            &output.push_str(line);
        }
    }
    
    output
    
}

pub fn manipulate_master_string(mut master_string: String, flags: CommandFlags) {
    //master_string = master_string.as_str();
    
    if flags.show_tabs {
        master_string = master_string.replace("\t", "^I");
    }

    if flags.show_ends {
        master_string = master_string.replace("\n", "$\n");
    }

    if flags.numbers {
        if flags.number_nonblanks {
            master_string = number_nonblanks(master_string);
        } else {
            master_string = number_lines(master_string);
        }
    }

    if flags.number_nonblanks {
        master_string = number_nonblanks(master_string);
    }

    println!("{}", master_string);
}
