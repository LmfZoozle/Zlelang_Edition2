#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(dead_code)]

use super::*;
use std::collections::HashSet;

//Operatorで列挙作ったのは我ながら賢いかも
//関数とかもシグニチャだとか定義だとか呼び出しだとか(オーバーロードだとか)
//めんどくさいし実装するかも
//それより先に enum name;を作って　
//関数の名前なのか、定数のなのか、変数のなのかとかってするほうがいいかも
//どっちをOperatorにしてどっちをOperatorsにするべきなのか問題　英語わからん
//anyとall?
//上、name よりident(識別子)のがいいか
pub enum Operators{
    Equal,
    Bigger,
    Smaller,
    CallFunc,
    Add,
    Sub,
    Mul,
    Div,
    Remain,
    Ref,
    Deref,
}

pub enum Tokens {
    Let,
    Var,
    VarName(String),
    Enum,
    EnumName(String),
    EnumElem(String),
    Option,
    For,
    While,
    Loop,
    Return,
    Class,
    ClassName(String),
    Label(String),
    Function,
    FunctionName(String),
    Operator(Operators),
}

static LITERAL_TOKEN: Vec<&str> = Vec::new();
//const IGNORE_LINE:&str="";

//あたまがおかしくなるよおおおおお！！！！！
//リテラルの' 'を無視してほしい
//でもリテラルの "" が片方欠損したときに処理がしぬ
//それならそれでいいのか？
//これはParseな気がするけど、とりま全部空白で切ろうか
//Parseに投げるぞ
//投げられないよ　これどうしよう
//matchにリテラル投げるの、ちょっと怖い
pub fn get_token_set(line: &mut String) -> Vec<Tokens> {
    let mut TokenSet = Vec::new();
    //ここはある種関数型みたいな記述なのかな？...
    let mut line: Vec<&str> = line.split_whitespace().collect();
    //エントリ発見はチェックしなくていいのか、多分
    //やっぱ最初の一回だけmain.zleにエントリあるか調べなきゃ
    //お前急にコメントめっちゃ書くなきっしょい
    //スタックの種類
    //(コメントアウト)
    //　"" リテラル
    // whileとかのブロック
    //let mut nest_stack=Vec::new();
    //let mut block_stack=Vec::new();
    for run in line {
        match run {
            "let" => TokenSet.push(Tokens::Let),
            "var" => TokenSet.push(Tokens::Var),
            //"fn" =>  TokenSet.push(Tokens::),
            //"main()" if !entry=>{
            //entry=true;
            //}
            //"\""=>,
            "while"=>TokenSet.push(Tokens::While),
            //"for"=>TokenSet.push(T),
            //"#unsafe"=>

            ":()" => {}
            _ => {}
        }
    }

    TokenSet
}
