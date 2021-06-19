use std::{env, fs, error::Error};

pub mod process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (file_vec, command_vec) = process::process_input(args[1..].to_vec());

    println!("{:?}\n{:?}", file_vec, command_vec);
}

