
pub mod lang;

use std::{env, process::exit, fs};

use lang::lexer;

fn get_contents(file_path: String) -> String {
    fs::read_to_string(file_path)
        .expect("Unable to read file {file_path}.")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Incorrect usage! Correct usage: ");
        println!("sable <input_file.sbl>");
        exit(1);
    }

    let file = args.get(1).unwrap();
    let contents = get_contents(file.clone());
    
    let tokens = lexer::tokenize(contents);
    println!("{:?}", tokens);

}
