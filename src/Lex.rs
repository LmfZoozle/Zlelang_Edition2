#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(dead_code)]

use super::*;
use std::collections::HashSet;



pub enum Tokens{
    Ident,
    KeyWord(keywords),
    Literal(literals),
    Operator(operators),
    Puncture(punches),
}

pub enum keywords {
    Let,
    Var,
    For,
    Fn,
    Return,
}

pub enum literals{
    Integer(i32),
    Double(f64),
    Char(char),
    Str(String),
}

pub enum operators{
    Equal,
    Bigger,
    Smaller,
    BorE,
    SorE,
    CallFunc,
    Add,
    Sub,
    Mul,
    Div,
    Remain,
    Ref,
    Deref,
}

pub enum punches {
    Bow,
    Round,
    Square,
}

pub fn get_token_set(line: &mut String) -> Vec<Tokens> {
    let mut TokenSet = Vec::new();
    //ここはある種関数型みたいな記述なのかな？...
    let mut line: Vec<&str> = line.split_whitespace().collect();

    TokenSet
}

