use std::fmt;
use std::fmt::*;

#[derive(Debug, PartialEq)]
pub enum Program {
    Func(Box<Function>),
    Decl(Box<Decl>),
    Expr(Box<Expr>), 
    Program(Vec<Box<Program>>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Function {
    Fn(String, Vec<Box<Argument>>, Type, Box<BlockExpr>),
    FnNoArg(String, Box<BlockExpr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    If(Box<Expr>, Box<BlockExpr>, Option<Box<BlockExpr>>),
    While(Box<Expr>, Box<BlockExpr>),
    Func(Box<Function>),
    Decl(Box<Decl>),
    Expr(Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BlockExpr {
    BlockExpr(Vec<Box<Stmt>>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Decl {
    Let(String, Option<Type>, Box<Expr>),
    LetMut(String, Option<Type>, Box<Expr>),
    Assign(String, Box<Expr>),
    AssignDeref(String, Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Argument {
    Argument(String, Type),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    NumOrId(NumOrId),
    NegativeNumOrId(Op, NumOrId),
    NegativeNumOrIdPar(Op, Box<Expr>),
    FunctionCall(String, Vec<Box<Expr>>),
    Op(Box<Expr>, Op, Box<Expr>),
    ParExpr(Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    I32,
    Boolean,
    Unknown,
    Unit,
    Ref(Box<Type>),
    RefMut(Box<Type>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum NumOrId {
    Id(String),
    Num(i32),
    Bool(bool),
    Ref(String),
    RefMut(String),
    Deref(String),
    Unit,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Eq,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    Not,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::I32 => write!(f, "{}", "i32")?,
            Type::Boolean => write!(f, "{}", "bool")?,
            Type::Unknown => write!(f, "{}", "Unknown type")?,
            Type::Unit => write!(f, "{}", "Unit")?,
            Type::Ref(t) => write!(f, "&{}", t)?,
            Type::RefMut(t) => write!(f, "&mut {}", t)?,
        };
        Ok(())
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Add => write!(f, "{}", "+")?,
            Op::Sub => write!(f, "{}", "-")?,
            Op::Mul => write!(f, "{}", "*")?,
            Op::Div => write!(f, "{}", "/")?,
            Op::And => write!(f, "{}", "&&")?,
            Op::Or => write!(f, "{}", "||")?,
            Op::Not => write!(f, "{}", "!=")?,
            Op::Eq => write!(f, "{}", "==")?,
            Op::LessEq => write!(f, "{}", "<=")?,
            Op::GreaterEq => write!(f, "{}", ">=")?,
            Op::Less => write!(f, "{}", "<")?,
            Op::Greater => write!(f, "{}", ">")?,
        };
        Ok(())
    }
}