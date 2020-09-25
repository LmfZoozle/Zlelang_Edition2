use super::*;

pub enum ItemTypeList {
    Statement,
    Block,
    Expression,
    Item,
    Type,
}

pub enum Syntaxlist {
    VarDeclare,
    VarInit,
    Assignment,
    FuncCall,
    FuncSign,
    FuncDefine,
}