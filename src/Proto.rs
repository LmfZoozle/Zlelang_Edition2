use super::*;
use std::process::exit;

pub enum Binary {
    NUM(i32),
    EOF,
    OPE(Box<Binary>, Box<Binary>, Ope),
}

//staticはやめよう
//static mut GEN:Binary=

pub fn new_ope(left: Box<Binary>, right: Box<Binary>, op: Ope) -> Box<Binary> {
    Box::new(Binary::OPE(left, right, op))
}

pub fn new_num(val: i32) -> Box<Binary> {
    Box::new(Binary::NUM(val))
}

pub fn expr(run: &mut std::slice::Iter<&str>) -> Box<Binary> {
    let mut node = mul(run);

    loop {
        let a = *run.next().unwrap();
        if a == "+" {
            node = new_ope(node, mul(run), Ope::ADD);
        } else if a == "-" {
            node = new_ope(node, mul(run), Ope::SUB);
        } else {
            return node;
        }
    }
}

pub fn mul(run: &mut std::slice::Iter<&str>) -> Box<Binary> {
    let mut node = primary(run);

    loop {
        let a = *run.next().unwrap();
        if a == "*" {
            node = new_ope(node, primary(run), Ope::MUL);
        } else if a == "/" {
            node = new_ope(node, primary(run), Ope::DIV);
        } else {
            return node;
        }
    }
}

pub fn primary(run: &mut std::slice::Iter<&str>) -> Box<Binary> {
    let a=*run.next().unwrap();

    if a=="("{
        return expr(run);
    }else{
        return new_num(7);
    }
}

pub fn ex_into_token(line: &String) -> Box<Binary> {
    let mut thisnode = Box::new(Binary::EOF);
    let mut spilt = line.split_whitespace().collect::<Vec<&str>>();
    let mut run = spilt.iter();
    loop {
        if let Some(content) = run.next() {
            match *content {
                _ if is_num(content) => {}
                "*" => {}
                _ => {}
            }
        } else {
            eprintln!("ファイル終端に到達");
        }
    }

    thisnode
}

pub enum TripleNode {
    NUM(i32),
    //parent,left,right
    OPE(
        Option<Box<TripleNode>>,
        Box<TripleNode>,
        Box<TripleNode>,
        Ope,
    ),
    EOF,
    EXPR(Box<TripleNode>),
}

/*
pub fn stacking_read_into_tokens(
    top: & Box<TripleNode>,
    line: String,
    mut now: usize,
) -> Box<TripleNode> {
    let mut thisnode = Box::new(TripleNode::EOF);
    let mut run = line.split_whitespace().collect::<Vec<&str>>();
    //let mut run = a.iter();
    now += 1;
    if let Some(content) = run.get(now - 1) {
        match *content {
            _ if is_num(&content) => {
                return Box::new(TripleNode::NUM(content.parse::<i32>().unwrap()));
            }
            "+" => {
                return Box::new(TripleNode::OPE(
                    None,
                    stacking_read_into_tokens(&thisnode, line.clone(), now - 2),
                    stacking_read_into_tokens(&thisnode, line, now),
                    Ope::ADD,
                ))
            }
            "-" => {
                return Box::new(TripleNode::OPE(
                    None,
                    stacking_read_into_tokens(&thisnode, line.clone(), now - 2),
                    stacking_read_into_tokens(&thisnode, line, now),
                    Ope::SUB,
                ))
            }
            //こっから違うぞ
            "*" => {
                return Box::new(TripleNode::OPE(
                    Some(top),
                    stacking_read_into_tokens(&mut thisnode, line.clone(), now - 2),
                    stacking_read_into_tokens(&mut thisnode, line, now),
                    Ope::MUL,
                ))
            }
            "-" => {}
            _ => {}
        }
    }
    thisnode
}*/
pub enum Ope {
    ADD,
    SUB,
    MUL,
    DIV,
}

pub enum Node {
    NUM(i32),
    OPE(Box<Node>, Box<Node>, Ope),
    EOF,
}

impl Node {
    pub fn get_value(&self) -> Option<i32> {
        if let Node::NUM(a) = self {
            Some(*a)
        } else {
            None
        }
    }
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

pub fn is_num(line: &str) -> bool {
    if let Ok(a) = line.parse::<i32>() {
        true
    } else {
        false
    }
}

//再帰的な型を再帰を使わずに処理出来るのか？
//とりあえずこの関数で木構造を作る
//もう一個デカい関数作ってそれでコード生成する
//再帰処理を使わずに再帰的な型を作れるのか？
//NUMならNODE作って付け足す　そしてブレイク

pub fn gen_code_to_beat_recursion(target: Box<Node>) {
    if let Node::NUM(a) = *target {
        println!("  push {}", a);
    }

    if let Node::OPE(a, b, c) = *target {
        gen_code_to_beat_recursion(a);
        gen_code_to_beat_recursion(b);

        match c {
            Ope::ADD => {
                println!("  add rax, rdi");
                return;
            }
            Ope::SUB => {
                println!("  add rax, rdi");
                return;
            }
            Ope::MUL => {
                println!("  add rax, rdi");
                return;
            }
            Ope::DIV => {
                println!("  add rax, rdi");
                return;
            }
        }
    }
    println!("  push rax");
}

//おつ　バグあるだろうけど
//->あった
//ひでなこれ　なんでnextを適用したし
//ちょっとこれ修正してやりなおす
//いやこれ木の深さも考えてないか　ダメだ
pub fn I_hate_recursion_but_create_tree(run_iter: &mut std::slice::Iter<&str>) -> Box<Node> {
    if let Some(content) = run_iter.next() {
        match *content {
            _ if is_num(content) => {
                return num_new(content.parse::<i32>().unwrap());
            }
            "+" => {
                let Left = I_hate_recursion_but_create_tree(run_iter);
                let Right = I_hate_recursion_but_create_tree(run_iter);
                return ope_new(Left, Right, Ope::ADD);
            }
            "-" => {
                let Left = I_hate_recursion_but_create_tree(run_iter);
                let Right = I_hate_recursion_but_create_tree(run_iter);
                return ope_new(Left, Right, Ope::SUB);
            }
            "*" => {
                let Left = I_hate_recursion_but_create_tree(run_iter);
                let Right = I_hate_recursion_but_create_tree(run_iter);
                return ope_new(Left, Right, Ope::MUL);
            }
            "/" => {
                let Left = I_hate_recursion_but_create_tree(run_iter);
                let Right = I_hate_recursion_but_create_tree(run_iter);
                return ope_new(Left, Right, Ope::DIV);
            }
            _ => {
                eprintln!("Unknown");
                exit(99)
            }
        }
    } else {
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
