use std::{env, fs, error::Error};
use anyhow::Result;

pub mod process;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    //having the `args[1..]` is important because env::args() has the path of
    //the executable as the first argument
    let (file_vec, command_vec) = process::process_input(args[1..].to_vec());

    let massive_string = process::collect_files_to_string(file_vec)?;

    println!("{}", massive_string);

    Ok(())
}

