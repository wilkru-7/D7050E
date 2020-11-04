use std::collections::{HashMap};

use crate::ast_new::*;
use crate::type_env::*;

pub fn id_type(n: &NumOrId, var_env: &mut VarEnv) -> Result<Type, Error> {
    match n {
        NumOrId::Num(_) => Ok(Type::I32),
        NumOrId::Bool(_) => Ok(Type::Boolean),
        NumOrId::Id(s) => help(s, var_env),
        NumOrId::Ref(s) => help(s, var_env),
        NumOrId::RefMut(s) => {
            let mut var: Type = Type::Unknown;
            let mut mutable: bool = false;
            for map in var_env.iter() {
                if map.contains_key(s) {
                    let op_var = map.get(s);
                    match op_var {
                        Some(t) => {
                            var = t.0.clone();
                            mutable = t.1;
                        }
                        None => {}
                    }
                    break;
                }
            }
            if var != Type::Unknown && mutable {
                Ok(var)
            } else {
                Err(format!("Variable {} not found in this scope or reference not mutable", s))
            }                
        }
        NumOrId::Deref(s) => help(s, var_env),
        _ => unimplemented!()
    }
}

fn help(s: &String, var_env: &mut VarEnv) -> Result<Type, Error> {
    let mut var: Type = Type::Unknown;
    for map in var_env.iter() {
        if map.contains_key(s) {
            let op_var = map.get(s);
            match op_var {
                Some(t) => {
                    var = t.0.clone();
                }
                None => {}
            }
            break;
        }
    }
    if var != Type::Unknown {
        Ok(var)
    } else {
        Err(format!("Variable {} not found in this scope", s))
    }    
}

pub fn expr_type(e: &Expr, var_env: &mut VarEnv, fn_env: &mut FnEnv) -> Result<Type, Error> {
    match e {
        Expr::NumOrId(n) => id_type(n, var_env),
        Expr::Op(l, op, r) => {
            
            let left = expr_type(l, var_env, fn_env)?;
            let right = expr_type(r, var_env, fn_env)?;
            
            match op {

                Op::Add | Op::Sub | Op::Div | Op::Mul => {
                    
                    if left == Type::I32 && right == Type::I32 {
                        Ok(Type::I32)
                    } else {
                        Err(format!("Mismatching types or unsupported operand - left: {}, right: {}, operand: {}", left, right, op))
                    }
                }

                Op::And | Op::Or => {
                    if left == Type::Boolean && right == Type::Boolean {
                        Ok(Type::Boolean)
                    }
                    else {
                        Err(format!("Mismatching types or unsupported operand - left: {}, right: {}, operand: {}", left, right, op))
                    }
                }
                
                Op::Greater | Op::Less | Op:: GreaterEq | Op::LessEq  => {
                    if left == Type::I32 && right == Type::I32 {
                        Ok(Type::Boolean)
                    }
                    else {
                        Err(format!("Mismatching types or unsupported operand - left: {}, right: {}, operand: {}", left, right, op))
                    }
                }

                Op::Eq | Op::Not => {
                    if left == Type::I32 && right == Type::I32 {
                        Ok(Type::Boolean)
                    } else if left == Type::Boolean && right == Type::Boolean {
                        Ok(Type::Boolean)
                    } else {
                        Err(format!("Mismatching types or unsupported operand - left: {}, right: {}, operand: {}", left, right, op))
                    }
                }
            }
        }
        Expr::ParExpr(e) => expr_type(e, var_env, fn_env),
        Expr::NegativeNumOrId(op, n) => {
            let right = id_type(n, var_env)?;
            match op {
                Op::Sub => {
                    if right == Type::I32 {
                        Ok(Type::I32)
                    } else {
                        Err(format!("Mismatching types or unsupported operand - right: {}, operand: {}", right, op))
                    }
                }
                Op::Not => {
                    if right == Type::Boolean {
                        Ok(Type::Boolean)
                    } else {
                        Err(format!("Mismatching types or unsupported operand - right: {}, operand: {}", right, op))
                    }
                }
                _ => unimplemented!()
            }
        }
        Expr::NegativeNumOrIdPar(op, e) => {
            let right = expr_type(e, var_env, fn_env)?;
            match op {
                Op::Sub => {
                    if right == Type::I32 {
                        Ok(Type::I32)
                    } else {
                        Err(format!("Mismatching types or unsupported operand - right: {}, operand: {}", right, op))
                    }
                }
                Op::Not => {
                    if right == Type::Boolean {
                        Ok(Type::Boolean)
                    } else {
                        Err(format!("Mismatching types or unsupported operand - right: {}, operand: {}", right, op))
                    }
                }
                _ => unimplemented!()
            }
        }
        Expr::FunctionCall(id, args) => {
            let mut fn_type = Type::Unknown;
            let mut flag2: bool = false;
            let mut flag: bool = false;
            if fn_env.contains_key(id) {
                match fn_env.get(id) {
                    Some(t) => {
                        fn_type = t.clone();
                    }
                    None => ()
                }
                let mut i: i32 = 1;
                for arg in args {
                    let e_type = expr_type(arg, var_env, fn_env)?;
                    let mut return_type: Type = Type::Unknown;
                    let s = id.to_string() + "-" + &i.to_string();
                    if fn_env.contains_key(&s) {
                        match fn_env.get(&s) {
                            Some(t) => {
                                return_type = t.clone();
                            }
                            None => {}
                        }
                        if return_type == e_type {
                            i = i + 1;
                            continue;
                        }
                        flag = true;
                        break;
                    } else {
                        flag2 = true;
                        break;
                    }
                }
                let s = id.to_string() + "-" + &i.to_string();
                if fn_env.contains_key(&s) {
                    flag2 = true;
                }
                if flag == true {
                    Err(format!("Wrong type for argument"))
                } else if flag2 == true {
                    Err(format!("Wrong number of arguments"))
                } else {
                    Ok(fn_type)
                }
            } else {
                Err(format!("Unknown variable: {}", id))
            }
        }
    }
}

pub fn block_type(block: &BlockExpr, var_env: &mut VarEnv, fn_env: &mut FnEnv) -> Result<Type, Error> {
    match block {
        BlockExpr::BlockExpr(stmts) => {
            push_front(var_env);
            let mut last = Type::Unknown;
            for stmt in stmts {
                last = stmt_type(stmt, var_env, fn_env)?;
                let statement = &**stmt;
                match statement {
                    Stmt::If(_, _, e) => {
                        match e {
                            Some(e) => {
                                last = block_type(e, var_env, fn_env)?;
                            }
                            None => {}
                        } 
                    }
                    Stmt::While(_, _) => {
                        last = Type::Unit;
                    }
                    Stmt::Func(_) => {
                        last = Type::Unit;
                    }
                    Stmt::Decl(_) => {
                        last = Type::Unit;
                    }
                    Stmt::Expr(e) => {
                        last = expr_type(e, var_env, fn_env)?;
                    }
                }
            }
            pop_front(var_env);
            Ok(last)
        }
    }
}

pub fn decl_type(d: &Decl, var_env: &mut VarEnv, fn_env: &mut FnEnv) -> Result<Type, Error> {
    match d {
        Decl::Let(id, t, e) => {
            let right = expr_type(e, var_env, fn_env)?;
            let expr_type = right.clone();
            if let Some(v) = var_env.get_mut(0) {
                v.insert(id.to_string(), (right, false));
            }
            match t {
                Some(t) => {
                    if expr_type == *t {
                        Ok(expr_type)
                    } else {
                        Err(format!("Mismatching types - left: {}, right: {}", t, expr_type))
                    }
                }
                None => Ok(expr_type)
            }
        }
        Decl::LetMut(id, t, e) => {
            let right = expr_type(e, var_env, fn_env)?;
            let expr_type = right.clone();
            if let Some(v) = var_env.get_mut(0) {
                v.insert(id.to_string(), (right, true));
            }
            match t {
                Some(t) => {
                    if expr_type == *t {
                        Ok(expr_type)
                    } else {
                        Err(format!("Mismatching types - left: {}, right: {}", t, expr_type))
                    }
                }
                None => Ok(expr_type)
            }
        }
        Decl::Assign(id, e) => {
            let right = expr_type(e, var_env, fn_env)?;
            let mut var: Type = Type::Unknown;
            let mut no_var: bool = false;
            let mut mutable: bool = false;
            for map in var_env.iter() {
                if map.contains_key(id) {
                    let op_var = map.get(id);
                    match op_var {
                        Some(t) => {
                            var = t.0.clone();
                            mutable = t.1;
                        }
                        None => {}
                    }
                    break;
                } else {
                    no_var = true;
                }
            }
            if var == right {
                if mutable == true {
                    Ok(var)
                } else {
                    Err(format!("{} is not a mutable variable", id))
                }
            } else if no_var == true {
                Err(format!("Illegal assign - variable {} do not exist", id))
            } else {
                Err(format!("Illegal assing -  variable has type {}, new type: {}", var, right))
            }        
        }
        Decl::AssignDeref(id, e) => {
            let right = expr_type(e, var_env, fn_env)?;
            let mut var: Type = Type::Unknown;
            let mut no_var: bool = false;
            // let mut mutable: bool = false;
            for map in var_env.iter() {
                if map.contains_key(id) {
                    let op_var = map.get(id);
                    match op_var {
                        Some(t) => {
                            var = t.0.clone();
                            // mutable = t.1;
                        }
                        None => {}
                    }
                    break;
                } else {
                    no_var = true;
                }
            }
            if var == right {
                // if mutable == true {
                    Ok(var)
                // } else {
                //     Err(format!("{} is not a mutable variable", id))
                // }
            } else if no_var == true {
                Err(format!("Illegal assign - variable {} do not exist", id))
            } else {
                Err(format!("Illegal assing -  variable has type {}, new type: {}", var, right))
            }             
        }
    }
}

pub fn stmt_type(s: &Stmt, var_env: &mut VarEnv, fn_env: &mut FnEnv) -> Result<Type, Error> {
    match s {
        Stmt::If(e, block, opt_else) => {
            let expr = expr_type(e, var_env, fn_env)?;
            let return_type = block_type(block, var_env, fn_env)?;
            let mut else_type = return_type.clone();
            match opt_else {
                Some(s) => {
                    else_type = block_type(s, var_env, fn_env)?;
                }
                None => {}
            }
            if expr == Type::Boolean {
                if else_type == return_type {
                    Ok(return_type)
                } else {
                    Err(format!("Mismatching types returned in if and else - if: {}, else: {}", return_type, else_type))
                }
            } else {
                Err(format!("Invalid expression in if - expected: {}, expression was: {}", Type::Boolean, expr))
            }
        }
        Stmt::While(e, block) => {
            let expr = expr_type(e, var_env, fn_env)?;
            let block = block_type(block, var_env, fn_env)?;
            if expr == Type::Boolean {
                Ok(block)
            } else {
                Err(format!("Invalid expression in while - expected: {}, expression was: {}", Type::Boolean, expr))
            }
        }
        Stmt::Func(f) => func_type(f, var_env, fn_env),
        Stmt::Decl(d) => decl_type(d, var_env, fn_env),
        Stmt::Expr(e) => expr_type(e, var_env, fn_env),
    }
}

pub fn func_type(f: &Function, var_env: &mut VarEnv, fn_env: &mut FnEnv) -> Result<Type, Error> {
    match f {
        Function::Fn(id, args, t, block) => {
            push_front(var_env);
            fn_env.insert(id.to_string(), t.clone());
           
            let mut i: i32 = 1;
            for arg in args {
                arg_insert(arg, var_env);
                arg_insert2(id, arg, fn_env, &i.to_string());
                i = i + 1;
            }
    
            let result = block_type(block, var_env, fn_env)?;
            pop_front(var_env);

            if *t == result {
                Ok(result)
            } else {
                Err(format!("Mismatching types - expected: {}, function returned: {}", t, result))
            }
        }
        Function::FnNoArg(id, block) => {
            fn_env.insert(id.to_string(), Type::Unit);
            let result = block_type(block, var_env, fn_env)?;
            let result2 = result.clone();
            fn_env.remove(id);
            fn_env.insert(id.to_string(), result);
            Ok(result2)
        }
    }
}

pub fn program_type(p: &Program, var_env: &mut VarEnv, fn_env: &mut FnEnv) -> Result<Type, Error> {
    match p {
        Program::Func(f) => func_type(f, var_env, fn_env),
        Program::Decl(d) => decl_type(d, var_env, fn_env),
        Program::Expr(e) => expr_type(e, var_env, fn_env),
        Program::Program(vec) => {
            push_front(var_env);
            for p in vec {
                program_type(p, var_env, fn_env)?;
            }
            pop_front(var_env);
            Ok(Type::Unit)
        }
    }
}

// fn push_front(env: &mut VecDeque<HashMap<String, Type>>) {
//     let var: HashMap<String, Type> = HashMap::new();
//     env.push_front(var);
// }

// fn pop_front(env: &mut VecDeque<HashMap<String, Type>>) {
//     env.pop_front();
// 
// }

fn push_front(env: &mut VarEnv) {
    let var: IdType = HashMap::new();
    env.push_front(var);
}

fn pop_front(env: &mut VarEnv) {
    env.pop_front();
}

fn arg_insert(a: &Argument, env: &mut VarEnv) {
    match a {
        Argument::Argument(s, t) => {
            let arg_type = t.clone();
            if let Some(v) = env.get_mut(0) {
                v.insert(s.to_string(), (arg_type, false));
            }
        }
    }
}   

fn arg_insert2(id: &String, a: &Argument, env: &mut FnEnv, i: &String) {
    match a {
        Argument::Argument(_s, t) => {
            let arg_type = t.clone();
            let name = id.to_string() + "-" + i;
            env.insert(name, arg_type);
        }
    }
}   