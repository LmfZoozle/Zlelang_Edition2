use super::*;
use std::collections::HashSet;
use std::process;

pub fn red_error() {
    eprint!("{}", ansi_term::Colour::Red.paint("Error: "));
}

pub fn yellow_warn() {
    eprint!("{}", ansi_term::Colour::Yellow.paint("Warning: "));
}

pub mod Fatal {
    use super::*;
    pub fn fatal_no_argument() {
        //print!("{}", ansi_term::Colour::Red.paint("Error: "));
        red_error();
        eprintln!("You need to set more than one argument.");
        eprintln!("Aborting due to previous error.");
        std::process::exit(1);
    }
    pub fn fatal_no_file() {
        //print!("{}", ansi_term::Colour::Red.paint("Error: "));
        red_error();
        eprintln!("There is no such a file.");
        eprintln!("Aborting due to previous error.");
        std::process::exit(1);
    }
    pub fn fatal_no_crate() {
        //print!("{}", ansi_term::Colour::Red.paint("Error: "));
        red_error();
        eprintln!("There is no such a crate.");
        eprintln!("Aborting due to previous error.");
        std::process::exit(1);
    }
}

pub mod Syntax {}

pub mod Warning {
    use super::*;

    pub fn wrong_file_extention(name: &str) {
        if let Some(_b) = name.find(".zle") {
            //do nothing
        } else {
            if let Some(_a) = name.find(".") {
                let mut extention = String::new();
                let mut find = false;
                for run in name.chars() {
                    if run == '.' {
                        find = true;
                    }
                    if find {
                        extention.push(run);
                    }
                }
                yellow_warn();
                eprintln!("Unexpected file extention ({}) .", extention);
                eprintln!("Check the file or change the extention into \".zle\" . ");
            } else {
                yellow_warn();
                eprintln!("File without any extention specified.");
                eprintln!("Expected \".zle\" ");
            }
        }
    }
}
