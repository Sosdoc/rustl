// This module contains comparison functions for numeric values

use lisp::env::Env;
use lisp::types::*;


// Adds this module's functions to the provided environment
pub fn add_module(env: &mut Env) {
    env.borrow_mut().insert("+".to_string(), RLType::Proc(add));
    env.borrow_mut().insert("-".to_string(), RLType::Proc(sub));
    env.borrow_mut().insert("*".to_string(), RLType::Proc(mul));
    env.borrow_mut().insert("/".to_string(), RLType::Proc(div));
}

fn add(args: Vec<RLType>) -> RLResult {
    if args.len() >= 2 {
        let mut sum: f32 = 0.0;
        for arg in args {
            sum += match arg {
                RLType::Number(n) => n,
                _ => return error(format!("Not a number: {}", arg)),
            }
        }
        Ok(RLType::Number(sum))
    } else {
        error(format!("Invalid number of arguments"))
    }
}

fn sub(args: Vec<RLType>) -> RLResult {
    if args.len() >= 2 {
        let mut result: f32 = match args[0] {
            RLType::Number(n) => n,
            _ => return error(format!("Not a number: {}", args[0])),
        };

        for i in 1..args.len() {
            result -= match args[i] {
                RLType::Number(n) => n,
                _ => return error(format!("Not a number: {}", args[i])),
            }
        }

        Ok(RLType::Number(result))
    } else {
        error(format!("Invalid number of arguments"))
    }
}


fn mul(args: Vec<RLType>) -> RLResult {
    if args.len() >= 2 {
        let mut total: f32 = 1.0;
        for arg in args {
            total *= match arg {
                RLType::Number(n) => n,
                _ => return error(format!("Not a number: {}", arg)),
            }
        }
        Ok(RLType::Number(total))
    } else {
        error(format!("Invalid number of arguments"))
    }
}

fn div(args: Vec<RLType>) -> RLResult {
    if args.len() >= 2 {
        let mut result: f32 = match args[0] {
            RLType::Number(n) => n,
            _ => return error(format!("Not a number: {}", args[0])),
        };

        for i in 1..args.len() {
            result /= match args[i] {
                RLType::Number(n) if n != 0.0 => n,
                _ => return error(format!("Not a number: {}", args[i])),
            }
        }

        Ok(RLType::Number(result))
    } else {
        error(format!("Invalid number of arguments"))
    }
}
