#[allow(non_snake_case)]
mod Error;
#[allow(non_camel_case_types)]
#[macro_use]
extern crate zoozle;
use zoozle::safe_scan;
//use zoozle::zle;
use std::fs;

//use Zle_Edition2::Error;
//version書き換えるの忘れずに。GoogleとかGithubにあげようね
//旧Zleもあげる？

fn main() {
    let version = "0.0.1";
    let test_path = String::from("/home/zoozle/Code/Rust/Zle_Edition2/example/");
    let defalut = "DEFAULT";

    echo!("Zle_2 Compiler version ", version, " Activated.");

    //let target_name = safe_scan!(defalut);

    let mut invalid = true;
    while invalid {
        let target_name: String = safe_scan!(defalut.to_string());
        let target_name: String = test_path.clone() + &target_name;
        let source = match fs::read_to_string(target_name.clone()) {
            Ok(a) => {
                //echo!("把握");
                invalid = false;
                a
            }
            Err(_a) => {
                echo!("No");
                Error::fatal_no_file();
                "INVALID".to_string()
            }
        };
    }
}
