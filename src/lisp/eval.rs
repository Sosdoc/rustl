use lisp::lex::{tokenize, parse_form};
use lisp::types::{RLType, RLResult, error};
use lisp::env::*;

pub fn eval(ast: RLType, env: &Env) -> RLResult {
    match ast {
        RLType::Symbol(ref name) => env.borrow().lookup(name),
        RLType::List(tokens) => eval_list(tokens, env),
        _ => Ok(ast),
    }
}

// Evaluates the list
// if the first element is a function or keyword, it executes that, otherwise returns
// the list itself
fn eval_list(mut tokens: Vec<RLType>, env: &Env) -> RLResult {
    // empty list -> no action
    if tokens.is_empty() {
        return Ok(RLType::List(tokens));
    }

    let first = tokens.remove(0);

    if let RLType::Symbol(name) = first {
        match eval_core(&name, &mut tokens, env) {
            Ok(value) => Ok(value),
            _ => eval_proc(name, tokens, env),
        }
    } else {
        // first is not a symbol, it's a regular list
        tokens.insert(0, first);
        Ok(RLType::List(tokens))
    }
}

fn eval_core(keyword: &str, args: &mut Vec<RLType>, env: &Env) -> RLResult {
    match keyword {
        "do" => eval_do(args, env),
        "if" => eval_if(args, env),
        "def!" => eval_def(args, env),
        // "quote" => eval_quote(args),
        // "lambda" => eval_lambda(args, env),
        _ => error("Unknown symbol."),
    }
}

fn eval_proc(name: String, tokens: Vec<RLType>, env: &Env) -> RLResult {
    if let Ok(RLType::Proc(func)) = env.borrow().lookup(&name) {
        // eager eval: each of the arguments is evaluated before calling
        let mut args: Vec<RLType> = Vec::new();
        for arg in tokens {
            match eval(arg, env) {
                Ok(value) => args.push(value),
                Err(e) => return Err(e),
            }
        }
    return func(args)
    }
    error("Unknown Symbol.")
}


// fn eval_lambda (args: &mut Vec<RLType>, env: &mut Environment) -> RLResult {
//     let mut arg_names : Vec<String> = Vec::new();
//     // get the list of arguments for the lambda
//     if let RLType::List(mut l_args) = args.remove(0) {
//         while l_args.len() > 0 {
//             if let RLType::Symbol(name) = l_args.remove(0) {
//                 arg_names.push(name);
//             }
//         }
//     }
//
//     let lambda_body = args.remove(0);
//
//     let lambda = move | exprs: RLType, outer: Environment| -> RLResult {
//         if let RLType::List(exp_vec) = exprs {
//             //let evaluated_args = exp_vec.iter().map(|e| eval(e, env));
//             let mut bindings : Vec<Binding> = Vec::new();
//
//             for (name, value) in arg_names.iter().zip(exp_vec) {
//                 bindings.push(Binding{key:(*name).to_string(), expr:value});
//             }
//
//             // RLType::Nil
//             let mut lambda_env = Environment::new_with_bindings(outer, bindings);
//
//             eval(lambda_body, &mut lambda_env)
//         } else {
//             Ok(RLType::Nil)
//         }
//     };
//     Ok(RLType::Nil)
// }

// Implementation for def
// usage: (def! name value ...)
fn eval_def(args: &mut Vec<RLType>, env: &Env) -> RLResult {
    // Check for a symbol as first argument
    if let RLType::Symbol(name) = args.remove(0) {
        match eval(args.remove(0), env) {
            Ok(value) => {
                env.borrow_mut().insert(name, value);
                Ok(RLType::Nil)
            },
            Err(e) => Err(e),
        }
    } else {
        error("Lookup key is not a symbol.")
    }
}

fn eval_do(args: &mut Vec<RLType>, env: &Env) -> RLResult {
    while args.len() > 1 {
        let term = args.remove(0);
        let _ = eval(term, env);
    }
    // eval and return last element
    eval(args.remove(0), env)
}

// Implementation for quote
// usage: (quote value ...)
// fn eval_quote(args: &mut Vec<RLType>) -> RLResult {
//     unimplemented!()
// }

// Implementation for if
// usage: (if test eval_if_true [eval_if_false])
fn eval_if(args: &mut Vec<RLType>, env: &Env) -> RLResult {
    let has_else = if args.len() == 3 {
        true
    } else {
        false
    };
    let condition = args.remove(0);

    match eval(condition, env) {
        Ok(RLType::True) => eval(args.remove(0), env),
        Ok(RLType::False) if has_else => eval(args.remove(1), env),
        Ok(_) => Ok(RLType::Nil),
        Err(e) => Err(e),
    }
}


// Parses the input &str and evals, for use in the REPL
pub fn parse_and_eval(input: &str, env: &Env) -> RLResult {
    let mut tokens = tokenize(input);
    let tree = parse_form(&mut tokens);
    // TODO: have descriptive error messages
    match tree {
        Ok(cell) => eval(cell, env),
        Err(_) => Ok(RLType::Symbol("parse error.".to_string())),
    }
}

// TODO: move tests in separate file
#[test]
fn eval_returns_pi() {
    let t = RLType::Symbol("pi".to_string());
    let mut env = Environment::default();

    let res = match eval(t, &mut env) {
        RLType::Number(n) => n,
        _ => 0.0,
    };

    assert_eq!(res, 3.14159265);
}


#[test]
fn eval_sum() {
    let args = vec![RLType::Symbol("+".to_string()), RLType::Number(2.0), RLType::Number(1.0)];

    let t = RLType::List(args);
    let mut env = Environment::default();

    let res = match eval(t, &mut env) {
        RLType::Number(n) => n,
        _ => 0.0,
    };

    assert_eq!(res, 3.0);

    let args = vec![RLType::Symbol("+".to_string()),
                    RLType::Number(3.0),
                    RLType::Number(2.0),
                    RLType::Number(1.0)];

    let t = RLType::List(args);
    let mut env = Environment::default();

    let res = match eval(t, &mut env) {
        RLType::Number(n) => n,
        _ => 0.0,
    };

    assert_eq!(res, 6.0);
}


#[test]
fn eval_sub() {
    let args = vec![RLType::Symbol("-".to_string()),
                    RLType::Number(3.0),
                    RLType::Number(2.0),
                    RLType::Number(1.0)];

    let t = RLType::List(args);
    let mut env = Environment::default();

    let res = match eval(t, &mut env) {
        RLType::Number(n) => n,
        _ => -10.0,
    };

    assert_eq!(res, 0.0);
}
