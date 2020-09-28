use super::*;
use std::process::exit;
pub enum Ope {
    ADD,
    SUB,
    MUL,
    DIV,
}
//頂点は任意子のポインタを持つ必要がある

pub enum Node {
    NUM(i32),
    OPE(Box<Node>, Box<Node>, Ope),
    EOF
}

/*pub fn new_node_ope(top:&mut Node,ty:Ope,lhs:Box<Node>,rhs:Box<Node>)->Box<Node>{
    top=&mut Node::OPE(lhs,rhs,ty);
    top
}*/

pub fn ope_new(left: Box<Node>, right: Box<Node>, op: Ope) -> Box<Node> {
    Box::new(Node::OPE(left, right, op))
}

pub fn num_new(val: i32) -> Box<Node> {
    Box::new(Node::NUM(val))
}


pub fn is_num(line:&str)->bool{
    if let Ok(a)=line.parse::<i32>(){
        true
    }else{
        false
    }
}

//再帰的な型を再帰を使わずに処理出来るのか？
//とりあえずこの関数で木構造を作る
//もう一個デカい関数作ってそれでコード生成する
//再帰処理を使わずに再帰的な型を作れるのか？
//NUMならNODE作って付け足す　そしてブレイク

//おつ　バグあるだろうけど
pub fn I_hate_recursion_but_create_tree(run_iter: &mut std::slice::Iter<&str>)->Box<Node>{
    if let Some(content) = run_iter.next() {
        
        match *content {
            _ if is_num(content)=>{
                return num_new(content.parse::<i32>().unwrap());
            }
            "+" => {
                let Left=I_hate_recursion_but_create_tree(run_iter);
                let Right=I_hate_recursion_but_create_tree(run_iter);
                return ope_new(Left, Right, Ope::ADD);
            }
            "-"=>{
                let Left=I_hate_recursion_but_create_tree(run_iter);
                let Right=I_hate_recursion_but_create_tree(run_iter);
                return ope_new(Left, Right, Ope::SUB);
            }
            "*"=>{
                let Left=I_hate_recursion_but_create_tree(run_iter);
                let Right=I_hate_recursion_but_create_tree(run_iter);
                return ope_new(Left, Right, Ope::MUL);
            }
            "/"=>{
                let Left=I_hate_recursion_but_create_tree(run_iter);
                let Right=I_hate_recursion_but_create_tree(run_iter);
                return ope_new(Left, Right, Ope::DIV);
            }
            _ => {
                eprintln!("Unknown");
                exit(99)
            }
        }
    }else{
        Box::new(Node::EOF)
    }
}

pub fn pop_front<T: Clone>(vec: &mut Vec<T>) -> T {
    let result = vec[0].clone();
    vec.remove(0);
    result
}

pub fn add_sub_mul_div(top: &mut Vec<&Node>, line: &String) {
    let mut que = Vec::new();
    let mut token_vec: Vec<&str> = line.split_whitespace().collect();
    let mut run_iter = token_vec.iter();
    let mut count = 0;
    loop {
        if let Some(t) = run_iter.next() {
            if count % 3 == 0 {
                if let Ok(a) = t.parse::<i32>() {
                    que.push(a);
                } else {
                    eprintln!("L_NAN:");
                    exit(78);
                }
            } else if count % 3 == 1 {
                match t {
                    &"+" => {}
                    &"-" => {}
                    &"*" => {}
                    &"/" => {}
                    _ => {
                        eprintln!("Unknown");
                        exit(556);
                    }
                }
            } else {
                if let Ok(a) = t.parse::<i32>() {
                    que.push(a);
                } else {
                    eprintln!("R_NAN:");
                    exit(78);
                }
            }
            count += 1;
        } else {
            break;
        }
    }
}

pub fn add_sub(line: &String) {
    let mut run: Vec<&str> = line.split_whitespace().collect();
    if let Ok(a) = run[0].parse::<i32>() {
        println!("mov rax, {}", run[0]);
    } else {
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
