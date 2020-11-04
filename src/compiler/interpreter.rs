use std::collections::{HashMap, VecDeque};

use crate::ast_new::*;
use crate::type_env::*;

pub fn id(n: &NumOrId, var_env: &mut VecDeque<HashMap<String, Option<NumOrId>>>) -> Result<NumOrId, Error> {
    match n {
        NumOrId::Num(m) => Ok(NumOrId::Num(*m)),
        NumOrId::Bool(m) => Ok(NumOrId::Bool(*m)),
        NumOrId::Id(s) => {
            let mut int: Option<i32> = None;
            let mut boolean: Option<bool> = None;
            for map in var_env.iter() {
                if map.contains_key(s) {
                    let op_var = map.get(s);
                    match op_var {
                        Some(t) => {
                            match t {
                                Some(n) => {
                                    match n {
                                        NumOrId::Id(_) => {
                                            continue;
                                        }
                                        NumOrId::Num(n) => {
                                            int = Some(*n);
                                        }
                                        NumOrId::Bool(n) => {
                                            boolean = Some(*n);
                                        }
                                        _ => unimplemented!()
                                    }
                                }
                                None => {}
                            }
                        }
                        None => {}
                    }
                    break;
                }
            }
            if let Some(value) = int {
                Ok(NumOrId::Num(value))
            } else if let Some(value) = boolean {
                Ok(NumOrId::Bool(value))
            } else {
                Err(format!("Variable1 {} not found in this scope", s))
            }
        }
        NumOrId::Ref(s) => {
            let mut reference: Option<NumOrId> = None;
            for map in var_env.iter() {
                if map.contains_key(s) {
                    reference = Some(NumOrId::Id(s.to_string()));
                }
            }
            if let Some(value) = reference {
                Ok(value)
            } else {
                Err(format!("Variable2 {} not found in this scope", s))
            }
        }
        NumOrId::RefMut(s) => {
            let mut reference: Option<NumOrId> = None;
            for map in var_env.iter() {
                if map.contains_key(s) {
                    reference = Some(NumOrId::Id(s.to_string()));
                }
            }
            if let Some(value) = reference {
                Ok(value)
            } else {
                Err(format!("Variable3 {} not found in this scope", s))
            }
        }
        NumOrId::Deref(s) => {
            // println!("{:?}", var_env);
            // let mut int: Option<i32> = None;
            // let mut boolean: Option<bool> = None;
            let mut result: Option<NumOrId> = None;
            for map in var_env.iter() {
                if map.contains_key(s) {
                    let op_var = map.get(s);
                    match op_var {
                        Some(t) => {
                            match t {
                                Some(n) => {
                                    match n {
                                        NumOrId::Id(s) => {
                                            result = Some(id(&NumOrId::Id(s.clone()), var_env)?);
                                            // result = Some(NumOrId::Id(s.clone()));
                                        }
                                        _ => unimplemented!()
                                    }
                                }
                                None => {}
                            }
                        }
                        None => {}
                    }
                    break;
                }
            }
            // if let Some(value) = int {
            //     Ok(NumOrId::Num(value))
            // } else if let Some(value) = boolean {
            //     Ok(NumOrId::Bool(value))
            if let Some(value) = result {
                Ok(value)
            } else {
                Err(format!("Reference for {} not found in this scope", s))
            }
        }
        _ => unimplemented!()
    }
}

pub fn expr(e: &Expr, vars: &mut VecDeque<HashMap<String, Option<NumOrId>>>, fn_env: &mut HashMap<String, Function>) -> Result<NumOrId, Error> {
    match e {
        Expr::NumOrId(n) => id(n, vars),
        Expr::Op(l, op, r) => {
            let left = expr(l, vars, fn_env)?;
            let right = expr(r, vars, fn_env)?;
            let mut left_int: i32 = 0;
            let mut right_int: i32 = 0;
            let mut left_bool: bool = false;
            let mut right_bool: bool = false;

            match left {
                NumOrId::Id(s) => {
                    for map in vars.iter() {
                        if map.contains_key(&s) {
                        let op_var = map.get(&s);
                        match op_var {
                            Some(t) => {
                                match t {
                                    Some(n) => {
                                        match n {
                                            NumOrId::Id(_) => {}
                                            NumOrId::Num(n2) => {
                                                left_int = *n2;
                                            }
                                            NumOrId::Bool(b) => {
                                                left_bool = *b;
                                            }
                                            _ => unimplemented!()
                                        }
                                    }
                                    None => {}
                                }
                            }
                            None => {}
                        }
                        break;
                        }
                    }
                }
                NumOrId::Num(n) => {
                    left_int = n;
                }
                NumOrId::Bool(b) => {
                    left_bool = b;
                }
                _ => unimplemented!()
            }
            match right {
                NumOrId::Id(s) => {
                    for map in vars.iter() {
                        if map.contains_key(&s) {
                        let op_var = map.get(&s);
                        match op_var {
                            Some(t) => {
                                match t {
                                    Some(n) => {
                                        match n {
                                            NumOrId::Id(_) => {}
                                            NumOrId::Num(n2) => {
                                                right_int = *n2;
                                            }
                                            NumOrId::Bool(b) => {
                                                right_bool = *b;
                                            }
                                            _ => unimplemented!()
                                        }
                                    }
                                    None => {}
                                }
                            }
                            None => {}
                        }
                        break;
                        }
                    }
                }
                NumOrId::Num(n) => {
                    right_int = n;
                }
                NumOrId::Bool(b) => {
                    right_bool = b;
                }
                _ => unimplemented!()
            }
            
            match op {
                Op::Add => {
                    Ok(NumOrId::Num(left_int + right_int))
                }
                Op::Sub => {
                    Ok(NumOrId::Num(left_int - right_int))
                }
                Op::Div => {
                    Ok(NumOrId::Num(left_int / right_int))
                }
                Op::Mul => {
                    Ok(NumOrId::Num(left_int * right_int))
                }
                Op::And => {
                    Ok(NumOrId::Bool(left_bool && right_bool))
                }
                Op::Or => {
                    Ok(NumOrId::Bool(left_bool || right_bool))
                }

                Op::Greater => {
                    Ok(NumOrId::Bool(left_int > right_int))
                }
                Op::GreaterEq => {
                    Ok(NumOrId::Bool(left_int >= right_int))
                }
                Op::Less => {
                    Ok(NumOrId::Bool(left_int < right_int))
                }
                Op::LessEq => {
                    Ok(NumOrId::Bool(left_int <= right_int))
                }
                // Op::Eq => {
                //     Ok(NumOrId::Bool(left_bool == right_bool))
                //     // Ok(NumOrId::Bool(left_int == right_int))
                // }
                // Op::Not => {
                //     Ok(NumOrId::Bool(left_bool != right_bool))
                //     // Ok(NumOrId::Bool(left_int == right_int))
                // }
                _ => unimplemented!()
            }
        }
        Expr::ParExpr(e) => expr(e, vars, fn_env),
        Expr::NegativeNumOrId(op, n) => {
            match op {
                Op::Sub => {
                    let left = id(n, vars)?;
                    let mut left_int: i32 = 0;

                    match left {
                        NumOrId::Id(_) => {}
                        NumOrId::Num(n) => {
                            left_int = n;
                        }
                        NumOrId::Bool(_) => {}
                        _ => unimplemented!()
                    }
                    Ok(NumOrId::Num(-left_int))
                }
                Op::Not => {
                    let left = id(n, vars)?;
                    let mut left_bool: bool = false;

                    match left {
                        NumOrId::Id(_) => {}
                        NumOrId::Num(_) => {}
                        NumOrId::Bool(b) => {
                            left_bool = b;
                        }
                        _ => unimplemented!()
                    }
                    Ok(NumOrId::Bool(!left_bool))
                }
                _ => unimplemented!() // Not possible operators
            }
        }
        Expr::NegativeNumOrIdPar(op, e) => {
            match op {
                Op::Sub => {
                    let left = expr(e, vars, fn_env)?;
                    let mut left_int: i32 = 0;

                    match left {
                        NumOrId::Id(_) => {}
                        NumOrId::Num(n) => {
                            left_int = n;
                        }
                        NumOrId::Bool(_) => {}
                        _ => unimplemented!()
                    }
                    Ok(NumOrId::Num(-left_int))
                }
                Op::Not => {
                    let left = expr(e, vars, fn_env)?;
                    let mut left_bool: bool = false;

                    match left {
                        NumOrId::Id(_) => {}
                        NumOrId::Num(_) => {}
                        NumOrId::Bool(b) => {
                            left_bool = b;
                        }
                        _ => unimplemented!()
                    }
                    Ok(NumOrId::Bool(!left_bool))
                }
                _ => unimplemented!() // Not possible operators
            }
        }
        Expr::FunctionCall(id, args) => {
            let fn_env2 = fn_env.clone();
            let mut result: Option<NumOrId> = None;           
            let mut i = 0;
            push_front(vars);
            for arg in args {
                let e = expr(arg, vars, fn_env)?;
                let mut id_arg: String = "".to_string();             
                match fn_env.get(id) {
                    Some(f) => {
                        match f {
                            Function::Fn(_, args2, _, _) => {
                                match &*args2[i] {
                                    Argument::Argument(s, _) => {
                                        id_arg = s.clone();
                                        
                                    }
                                }
                            }
                            Function::FnNoArg(_, _) => {}
                        }
                    }
                    None => {}
                }
                i = i + 1;
                if let Some(v) = vars.get_mut(0) {
                    v.insert(id_arg, Some(e));
                }
            }
            if fn_env2.contains_key(id) {
                match fn_env2.get(id) {
                    Some(f) => {
                        result = func(f, vars, fn_env)?;
                        pop_front(vars);
                    }
                    None => {}
                }
                match result {
                    Some(n) => {
                        println!("{:?}", n);
                        Ok(n)
                    }
                    None => {
                        Ok(NumOrId::Unit)
                        // Err(format!("No value returned"))
                    }
                }               
            } else {
                Err(format!("Can not find function {}", id))
            }
        }
    }
}

pub fn block_exe(block: &BlockExpr, var_env: &mut VecDeque<HashMap<String, Option<NumOrId>>>, fn_env: &mut HashMap<String, Function>) -> Result<Option<NumOrId>, Error> {
    match block {
        BlockExpr::BlockExpr(stmts) => {
            // println!("{:?} in block", var_env);
            push_front(var_env);
            let mut last = None;
            for stmt in stmts {
                last = stmt_exe(stmt, var_env, fn_env)?;
                let statement = &**stmt;
                match statement {
                    Stmt::If(_, _, _) => {
                        // last = None;
                    }
                    Stmt::While(_, _) => {
                        last = None;
                    }
                    Stmt::Func(_) => {
                        last = None;
                    }
                    Stmt::Decl(_) => {
                        last = None;
                    }
                    Stmt::Expr(_e) => {
                        // let temp = expr(e, var_env, fn_env)?;
                        // last = Some(temp);
                        // last = None;
                    }
                }
            }
            // println!("{:?} in block", var_env);
            pop_front(var_env);
            // println!("{:?}", last);
            Ok(last)
        }
    }
}

pub fn decl(d: &Decl, vars: &mut VecDeque<HashMap<String, Option<NumOrId>>>, fn_env: &mut HashMap<String, Function>) -> Result<Option<NumOrId>, Error> {
    match d {
        Decl::Let(id, _t, e) => {
            let right = expr(e, vars, fn_env)?;
            if let Some(v) = vars.get_mut(0) {
                v.insert(id.to_string(), Some(right));
            }
            Ok(None)
        }
        Decl::LetMut(id, _t, e) => {
            let right = expr(e, vars, fn_env)?;
            if let Some(v) = vars.get_mut(0) {
                v.insert(id.to_string(), Some(right));
            }
            Ok(None)
        }
        Decl::Assign(id, e) => {
            let right: NumOrId = expr(e, vars, fn_env)?;
            // println!("{:?}", vars);
            for var in vars {
                if var.contains_key(id){
                    var.remove(id);
                    var.insert(id.clone(), Some(right));
                    break;
                }
            }
            Ok(None)    
        }
        Decl::AssignDeref(id, e) => {
            let right: NumOrId = expr(e, vars, fn_env)?;
            // println!("{:?} for {}", vars, id);
            let mut string: &String = &"".to_string();
            for var in vars {
                if string == &"".to_string() {
                    // let temp: HashMap<String, Option<NumOrId>> = var.clone();
                    if var.contains_key(id){
                        let op_var = var.get(id);
                        match op_var {
                            Some(t) => {
                                match t {
                                    Some(n) => {
                                        match n {
                                            NumOrId::Id(s) => {
                                                string = s;
                                            }
                                            _ => unimplemented!()
                                        }
                                    }
                                    None => {}
                                }
                            }
                            None => {}
                        }
                        // var.remove(string);
                        // var.insert(string.clone(), Some(right)); 
                    }
                } 
                else {
                    if var.contains_key(string){
                        var.remove(string);
                        var.insert(string.clone(), Some(right));
                        break;
                    }
                }
            }
            Ok(None) 
        }
    }
}

pub fn stmt_exe(s: &Stmt, var_env: &mut VecDeque<HashMap<String, Option<NumOrId>>>, fn_env: &mut HashMap<String, Function>) -> Result<Option<NumOrId>, Error> {
    match s {
        Stmt::If(e, block, opt_else) => {
            let expr = expr(e, var_env, fn_env)?;
            if expr == NumOrId::Bool(true) {
                let return_type = block_exe(block, var_env, fn_env)?;
                Ok(return_type)
            } else {
                let mut else_type = None;
                match opt_else {
                    Some(s) => {
                        else_type = block_exe(s, var_env, fn_env)?;
                    }
                    None => {}
                }
                Ok(else_type)
            }
        }
        Stmt::While(e, block) => {
            let start_expr = expr(e, var_env, fn_env)?;
            let mut expr_bool = false;
            match start_expr {
                NumOrId::Id(_) => {}
                NumOrId::Num(_) => {}
                NumOrId::Bool(b) => {
                    expr_bool = b;
                }
                _ => unimplemented!()
            }
            while expr_bool {
                block_exe(block, var_env, fn_env)?;
                let temp = expr(e, var_env, fn_env)?;
                match temp {
                    NumOrId::Id(_) => {}
                    NumOrId::Num(_) => {}
                    NumOrId::Bool(b) => {
                        expr_bool = b;
                    }
                    _ => unimplemented!()
                }
            }
            Ok(None)
        }
        Stmt::Func(f) => func(f, var_env, fn_env),
        Stmt::Decl(d) => decl(d, var_env, fn_env),
        Stmt::Expr(e) => {
            let result = expr(e, var_env, fn_env)?;
            Ok(Some(result))
        }
    }
}

pub fn func(f: &Function, var_env: &mut VecDeque<HashMap<String, Option<NumOrId>>>, fn_env: &mut HashMap<String, Function>) -> Result<Option<NumOrId>, Error> {
    match f {
        Function::Fn(_, _args, _t, block) => {
            let result = block_exe(block, var_env, fn_env)?;
            // fn_env.remove(id);
            Ok(result)
        }
        Function::FnNoArg(_, block) => {
            // fn_env.insert(id.to_string(), None);
            let result = block_exe(block, var_env, fn_env)?;
            // fn_env.remove(id);
            Ok(result)
        }
    }
}

pub fn program(p: &Program, var_env: &mut VecDeque<HashMap<String, Option<NumOrId>>>, fn_env: &mut HashMap<String, Function>) -> Result<Option<NumOrId>, Error> {
    match p {
        Program::Func(f) => func(f, var_env, fn_env),
        Program::Decl(d) => decl(d, var_env, fn_env),
        Program::Expr(e) => {
            let result = expr(e, var_env, fn_env)?;
            Ok(Some(result))
        },
        Program::Program(vec) => {
            push_front(var_env);
            let mut main: bool = false;
            for p in vec {
                match &**p {
                    Program::Func(f) => {
                        let f2 = f.clone();
                        match &**f {
                            Function::Fn(s, _, _, _) => {
                                let s2 = s.clone();
                                fn_env.insert(s2, *f2);
                                if s == "main" {
                                    func(f, var_env, fn_env)?;
                                    main = true;
                                }    
                            }
                            Function::FnNoArg(s, _) => {
                                let s2 = s.clone();
                                fn_env.insert(s2, *f2);
                                if s == "main" {
                                    func(f, var_env, fn_env)?;
                                    main = true;
                                }
                            }
                        }
                    }
                    Program::Decl(_) => {}
                    Program::Expr(_) => {}
                    Program::Program(_) => {}
                }
            }
            // println!("{:?}", var_env);
            pop_front(var_env);
            if main == true {
                Ok(None)
            } else {
                Err(format!("No main found"))
            }
        }
    }
}

fn push_front(env: &mut VecDeque<HashMap<String, Option<NumOrId>>>) {
    let var: HashMap<String, Option<NumOrId>> = HashMap::new();
    env.push_front(var);
}

fn pop_front(env: &mut VecDeque<HashMap<String, Option<NumOrId>>>) {
    env.pop_front();

}