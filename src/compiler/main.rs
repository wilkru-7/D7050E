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

//Not sure about when to have ;
#[test]
fn type_check() -> Result<(), String> {
    let mut var_env = VecDeque::new();
    let mut var_env2 = VecDeque::new();
    let mut fn_env: HashMap<String, Type> = HashMap::new();
    let mut fn_env2: HashMap<String, Function> = HashMap::new();

    // println!("{:?}", ProgramParser::new().parse("
    // fn a(x1: bool, y1: bool) -> bool {
    //     x1 && y1
    // };
    // fn b(x2: bool, y2: bool) -> i32 {
    //     let a: bool = a(x2, y2 || false);
    //     let mut b: i32 = 0;
    //     if a && y2 {
    //         let a: bool = true; // shadowing
    //         if y2 || a {
    //             b = b + 1;
    //         };
    //     } else {
    //         if !(x2 && false) {
    //             b = b - 1;
    //         }
    //     };
    //     b + 3
    // }
    // ").unwrap());
 
    // let x = ProgramParser::new().parse("
    // fn a(x2: bool, y2: bool) -> bool {
    //     x2 || y2
    // };
    // fn b(x1: bool, y1: bool) -> i32 {
    //     let a: bool = a(x1, y1 || false);
    //     let mut b: i32 = 0;
    //     if a && y1 {
    //         let a: bool = true; // shadowing
    //         if y1 || a {
    //             b = b + 1;
    //         };
    //     } else {
    //         if !(a && false) {
    //             b = b - 1;
    //         }
    //     };
    //     b + 3
    // };
    // fn c(y: bool) -> bool {
    //     false
    // }
    // ").unwrap();

    let x = ProgramParser::new().parse("
    fn a(x: bool, y: bool) -> bool {
        if x && y {
            let a: bool = true;
            y || a
        } else {
            x && false
        }
    };
    fn b(x: bool, y: bool) -> i32 {
        let a: bool = a(x, y || false);
        let mut b: i32 = 0;
        if a && y {
            let a: bool = true; // shadowing
            if y || a {
                b = b + 1;
            }
        } else {
            if !(x && false) {
                b = b - 1;
            }
        };
        b + 3
    };
    fn main() -> () {
        let result: i32 = b(true, true);
    }
    ").unwrap();
    
    assert_eq!(type_check::program_type(&x, &mut var_env, &mut fn_env)?, Type::Unit);
    assert_eq!(interpreter::program(&x, &mut var_env2, &mut fn_env2)?, None);
    Ok(())    
}

// Not implemented:
// " " in comments
// ; not needed between functions
// Later defined functions 
// Nested functions
// References
// Borrow checking
// Global let
// While no return type
// Shadowing
// Let without type
// Var envs for decl/expr only
// Interpretern hanterar inte rätt scopes men fångas i type_checkern
// Functions needed in specific order in type_check