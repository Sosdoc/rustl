// This module contains comparison functions for numeric values

use lisp::env::Env;
use lisp::types::*;

// Adds this module's functions to the provided environment
pub fn add_module(env: &mut Env) {
    env.borrow_mut().insert(">".to_string(), RLType::Proc(gt));
    env.borrow_mut().insert(">=".to_string(), RLType::Proc(gte));
    env.borrow_mut().insert("<".to_string(), RLType::Proc(lt));
    env.borrow_mut().insert("<=".to_string(), RLType::Proc(lte));
    env.borrow_mut().insert("=".to_string(), RLType::Proc(eq));
}

fn gt(args: Vec<RLType>) -> RLResult {
    if args.len() == 2 {
        match check_two_numbers(args) {
            Ok((left, right)) => {
                if left > right {Ok(RLType::True)} else {Ok(RLType::False)}
            },
            Err(e) => Err(e),
        }
    } else {
        error("Invalid number of arguments")
    }
}

fn gte(args: Vec<RLType>) -> RLResult {
    if args.len() == 2 {
        match check_two_numbers(args) {
            Ok((left, right)) => {
                if left >= right {Ok(RLType::True)} else {Ok(RLType::False)}
            },
            Err(e) => Err(e),
        }
    } else {
        error("Invalid number of arguments")
    }
}

fn lt(args: Vec<RLType>) -> RLResult {
    if args.len() == 2 {
        match check_two_numbers(args) {
            Ok((left, right)) => {
                if left < right {Ok(RLType::True)} else {Ok(RLType::False)}
            },
            Err(e) => Err(e),
        }
    } else {
        error("Invalid number of arguments")
    }
}

fn lte(args: Vec<RLType>) -> RLResult {
    if args.len() == 2 {
        match check_two_numbers(args) {
            Ok((left, right)) => {
                if left <= right {Ok(RLType::True)} else {Ok(RLType::False)}
            },
            Err(e) => Err(e),
        }
    } else {
        error("Invalid number of arguments")
    }
}

fn eq(args: Vec<RLType>) -> RLResult {
    if args.len() == 2 {
        match check_two_numbers(args) {
            Ok((left, right)) => {
                if left == right {Ok(RLType::True)} else {Ok(RLType::False)}
            },
            Err(e) => Err(e),
        }
    } else {
        error("Invalid number of arguments")
    }
}

fn check_two_numbers(args: Vec<RLType>) -> Result<(f32, f32), RLError> {
    let mut numbers = Vec::new();

    for arg in args {
        match arg {
            RLType::Number(n) => numbers.push(n),
            _ => return Err(RLError::Message("Not a number, cannot compare.".to_string())),
        }
    }

    Ok((numbers[0], numbers[1]))
}
