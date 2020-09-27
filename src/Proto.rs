use super::*;
use std::process::exit;
pub enum Token {
    ADD,
    SUB,
}

pub fn add_sub(line: &String) {
    let mut run: Vec<&str> = line.split_whitespace().collect();
    if let Ok(a)=run[0].parse::<i32>(){
        println!("mov rax, {}", run[0]);
    }else{
        eprintln!("First_Invalid:Abort!");
        exit(88);
    }
    run.remove(0);
    let mut it = run.iter();
    loop {
        match it.next() {
            Some(&"+") => {
                eprintln!("ADD calc");
                if let Some(a) = it.next() {
                    if let Ok(b) = a.parse::<i32>() {
                        println!("add rax, {}", b);
                    } else {
                        eprintln!("Not a number:Abort!");
                        std::process::exit(5);
                    }
                } else {
                    eprintln!("Lack a Arg:Abort!");
                    std::process::exit(9);
                }
            }
            Some(&"-") => {
                eprintln!("SUB calc");
                if let Some(a) = it.next() {
                    if let Ok(b) = a.parse::<i32>() {
                        println!("sub rax, {}", b);
                    } else {
                        eprintln!("Not a number:Abort!");
                        std::process::exit(5);
                    }
                } else {
                    eprintln!("Lack a Arg:Abort!");
                    std::process::exit(9);
                }
            }
            None => {
                break;
            }
            _ => {
                eprintln!("UnEx:Abort!");
                std::process::exit(3);
            }
        }
    }
}
