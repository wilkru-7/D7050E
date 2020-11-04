use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/compiler/parser_new.rs");

use parser::*;
pub use crate::ast_new::*;
use std::collections::{HashMap, VecDeque};

pub mod ast_new;
pub mod type_env;
pub mod type_check;
pub mod interpreter;

fn main() {
    println!("minimal");
    println!("{:?}", NumOrIdParser::new().parse("123"));
    println!("{:?}", NumOrIdParser::new().parse("a1_a"));
}


#[test]
fn type_check() -> Result<(), String> {
    let mut var_env = VecDeque::new();
    let mut fn_env: HashMap<String, Type> = HashMap::new();

    println!("{:?}", ProgramParser::new().parse("
    fn a(x1: bool, y1: bool) -> bool {
        x1 && y1
    };
    fn b(x2: bool, y2: bool) -> i32 {
        let a: bool = a(x2, y2 || false);
        let mut b: i32 = 0;
        if a && y2 {
            let a: bool = true; // shadowing
            if y2 || a {
                b = b + 1;
            };
        } else {
            if !(x2 && false) {
                b = b - 1;
            }
        };
        b + 3
    }").unwrap());

    let x = ProgramParser::new().parse("
    fn a(x1: bool, y1: bool) -> bool {
        x1 && y1
    };
    fn b(x2: bool, y2: bool) -> i32 {
        let a: bool = a(x2, y2 || false);
        let mut b: i32 = 0;
        if a && y2 {
            let a: bool = true; // shadowing
            if y2 || a {
                b = b + 1;
            };
        } else {
            if !(x2 && false) {
                b = b - 1;
            }
        };
        b + 3
    }
    ").unwrap();
    
    assert_eq!(type_check::program_type(&x, &mut var_env, &mut fn_env)?, Type::Unit);
    Ok(())    
}

#[test]
fn test1() -> Result<(), String> {
    let mut var_env = VecDeque::new();
    let mut var_env2 = VecDeque::new();
    let mut fn_env: HashMap<String, Type> = HashMap::new();
    let mut fn_env2: HashMap<String, Function> = HashMap::new();
 
    let x = ProgramParser::new().parse("
    fn a(x: i32, y: i32) -> i32 {
        let mut a: i32 = x;
        let mut b: i32 = y;
        while b > a {
            a = a + 1;
        };
        a + b;
    };  
    fn main() -> () {
        a(3, 2);
    }
    ").unwrap();
    
    assert_eq!(type_check::program_type(&x, &mut var_env, &mut fn_env)?, Type::Unit);
    assert_eq!(interpreter::program(&x, &mut var_env2, &mut fn_env2)?, None);
    Ok(())    
}

#[test]
fn test2() -> Result<(), String> {
    let mut var_env = VecDeque::new();
    let mut var_env2 = VecDeque::new();
    let mut fn_env: HashMap<String, Type> = HashMap::new();
    let mut fn_env2: HashMap<String, Function> = HashMap::new();

    let x = ProgramParser::new().parse("
    fn test1(x: i32, y: i32) -> i32 {
        if x < y {
            if x > 0 {
                - x + (2 * y)
            };
            x + y
        } else {
            - x - (2 * y)
        }
    };
    fn test2(x: bool) -> bool {
        let mut a: i32 = 3; 
        let mut b: bool = false;
        let c: i32 = test1(a, 10/2);
        while a >= 0 {
            b = !b;
            a = a - 1;
        };
        b
    };
    fn main() -> () {
        let start: bool = true;
        test2(start || false);
    }
    ").unwrap();
    
    assert_eq!(type_check::program_type(&x, &mut var_env, &mut fn_env)?, Type::Unit);
    assert_eq!(interpreter::program(&x, &mut var_env2, &mut fn_env2)?, None);
    Ok(())    
}

#[test]
fn test3() -> Result<(), String> {
    let mut var_env = VecDeque::new();
    let mut var_env2 = VecDeque::new();
    let mut fn_env: HashMap<String, Type> = HashMap::new();
    let mut fn_env2: HashMap<String, Function> = HashMap::new();

    let x = ProgramParser::new().parse("
    fn b(x: &mut i32, y: &mut i32) -> () {
        *x = *x + 1;
        *y = *y + 1;
    };
    fn main() -> () {
        let mut x: i32 = 5;
        let mut y: i32 = 5;
        b(&mut x, &mut y);
    }
    ").unwrap();
    
    assert_eq!(type_check::program_type(&x, &mut var_env, &mut fn_env)?, Type::Unit);
    assert_eq!(interpreter::program(&x, &mut var_env2, &mut fn_env2)?, None);
    Ok(())    
}