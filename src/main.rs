#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(dead_code)]

use ansi_term::Colour::*;
#[macro_use]
extern crate zoozle;
use zoozle::safe_scan;

use std::env::args;
use std::fs;
use std::process::Command;

mod Error;
mod Lex;
mod Parse;
//mod Optimize;
use Error::Fatal;
use Error::Syntax;
use Error::Warning;

const WHERE_PATH: &str = "/home/zoozle/Code/Rust/Zle_Edition2/";

const TEST_PATH: &str = "/home/zoozle/Code/Rust/Zle_Edition2/example/";
const STDLIB_PATH: &str = "/home/zoozle/Code/Rust/Zle_Edition2/ZleStdLib/";

static mut ABORT: bool = false;
const EDITION: &str = "nightly";

const DEBUG: bool = true;

fn main() {
    let env_args: Vec<String> = std::env::args().collect();
    let version = "0.0.1";
    let mut source_code = String::new();

    echo!("Zle_2 Compiler version ", version, " Activated.");
    if env_args.len() <= 1 {
        Fatal::fatal_no_argument();
    }

    if "nightly" == EDITION {
        println!(
            "{} This is {} build. ",
            Red.paint("Attention!"),
            Red.paint(EDITION)
        );
    }

    if DEBUG {
        if let Ok(a) = fs::read_to_string(format!("{}{}", &TEST_PATH, &env_args[1])) {
            source_code = a;
            println!("File recognized. Please wait...");
            Warning::wrong_file_extention(&env_args[1]);
        } else {
            Fatal::fatal_no_file();
        }
    }else{
        if let Ok(a) = fs::read_to_string( &env_args[1]) {
            source_code = a;
            println!("File recognized. Please wait...");
            Warning::wrong_file_extention(&env_args[1]);
        } else {
            Fatal::fatal_no_file();
        }
    }    
}
