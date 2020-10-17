use std::fmt;
use std::fmt::*;

struct DisplayOption<T>(pub Option<T>);
struct DisplayVec<T>(pub Vec<T>);

// pub type Error = String;

// pub type IdType = HashMap<String, Type>;

// pub type VarEnv = VecDeque<IdType>;

// #[derive(Debug)]
// pub struct VarEnv(VecDeque<IdType>);

#[derive(Debug)]
pub enum Stmt {
    Let(String, Option<Type>, Box<Expr>, Option<Box<Stmt>>),
    Mut(String, Option<Type>, Box<Expr>, Option<Box<Stmt>>),
    Fn(String, Vec<Box<Argument>>, Type, Box<Stmt>, Option<Box<Stmt>>),
    NoArgsFn(String, Box<Stmt>, Option<Box<Stmt>>),
    If(Box<Expr>, Vec<Box<Stmt>>, Option<Box<Stmt>>),
    ElseIf(Box<Expr>, Vec<Box<Stmt>>, Option<Box<Stmt>>),
    Else(Vec<Box<Stmt>>, Option<Box<Stmt>>),
    While(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    Assign(String, Box<Expr>),
    Expr(Box<Expr>),
}

#[derive(Debug)]
pub enum Argument {
    Argument(String, Type),
}

#[derive(Debug, PartialEq)]
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
    NoType,
    //Mut<Box<Type>>,
}

#[derive(Debug, PartialEq)]
pub enum NumOrId {
    Num(i32),
    Id(String),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
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



impl<T: Display> Display for DisplayOption<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.0 {
            Some(ref v) => write!(f, "Some({})", v),
            None => write!(f, "None")
        }
    }
}

impl<T: Display> Display for DisplayVec<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut comma_separated = String::new();

        for num in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&num.to_string());
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
        write!(f, "{}", comma_separated)
    }
}

// impl fmt::Display for Stmt {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Stmt::Let(s, t, e, d) => write!(f, "let {}: {} {} \n {}", s, t, e, d)?,
//             Stmt::Mut(s, t, e, d) => write!(f, "let mut{}: {} {} \n {}", s, t, e, d)?,
//             Stmt::Fn(s, a, t, d) => write!(f, "fn {} {} {} {{\n {} }}", s, a, t, d)?,
//             Stmt::NoArgsFn(s, d) => write!(f, "{}\n {}", s, d)?,
//             Stmt::If(e, d, s) => write!(f, "if {} {} {}", e, d, s)?,
//             Stmt::ElseIf(s, d) => write!(f, "else if {} {}", s, d)?,
//             Stmt::Else(s, d) => write!(f, "else {} {}", s, d)?,
//             Stmt::While(e, s, d) => write!(f, "while {} {} {}", e, s, d)?,
//             Stmt::Assign(s, e) => write!(f, "{} {}", s, e)?,
//             Stmt::Expr(e) => write!(f, "{}", e)?,
//         };
//         Ok(())
//     }
// }

impl fmt::Display for NumOrId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumOrId::Num(i) => write!(f, "{}", i)?,
            NumOrId::Id(s) => write!(f, "{}", s)?,
            NumOrId::Bool(b) => write!(f, "{}", b)?,
        };
        Ok(())
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Argument::Argument(s, t) => write!(f, "{} {}", s, t)?,
        };
        Ok(())
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::I32 => write!(f, "{}", "i32")?,
            Type::Boolean => write!(f, "{}", "bool")?,
            Type::NoType => write!(f, "{}", "No type")?,
        };
        Ok(())
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::NumOrId(n) => write!(f, "{}", n)?,
            Expr::NegativeNumOrId(op, n) => write!(f, "{} {}", op, n)?,
            Expr::NegativeNumOrIdPar(op, exp) => write!(f, "{} {}", op, exp)?,
            Expr::FunctionCall(_id, _a) => write!(f, "")?,
            Expr::Op(ref left, op, ref right) => write!(f, "{} {} {}", left, op, right)?, 
            Expr::ParExpr(e) => write!(f, "({})", e)?,
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