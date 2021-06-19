use std::{fs};

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
///The core of the "cat" command, reading files, concatenating them
///and pushing the result to stdout
///
///Note that if one of these files is `-` then it will continuously read
///from stdin and print that on stdout, without a way to exit the program
pub fn print_files(files: Vec<String>) {
    let master_string = String::new();

    for i in files {
        if i == String::from("-"){
            cat_stdin();
        
        let file_string: String = fs::read_to_string(i);
        master_string.push_str(&file_string);
        }
    }
}

}
