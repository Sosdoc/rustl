use lisp::lex::{tokenize, parse_form};
use lisp::types::*;
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

    let mut first = tokens.remove(0);

    if !first.is_atom() {
        first = match eval(first, env) {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    }

    match first {
        RLType::Symbol(name) => {
            if let Ok(value) = eval_core(&name, &mut tokens, env) {
                return Ok(value);
            }
            eval_proc(&name, tokens, env)
        },
        RLType::Lambda(lambda) => {
            match make_atomic(tokens, env) {
                Ok(RLType::List(ref mut args)) => eval_exec_lambda(lambda, args),
                Ok(v) => error(format!("args are not a list: {}", v)),
                Err(e) => Err(e),
            }
        }
        _ => error(format!("Element is not a function: {}", first)),
    }
}

fn eval_core(keyword: &str, args: &mut Vec<RLType>, env: &Env) -> RLResult {
    match keyword {
        "do" => eval_do(args, env),
        "if" => eval_if(args, env),
        "def!" => eval_def(args, env),
        "lambda" => eval_create_lambda(args, env),
        "list" => eval_make_list(args, env),
        _ => error(format!("Not a keyword: {}", keyword)),
    }
}

fn eval_proc(name: &str, tokens: Vec<RLType>, env: &Env) -> RLResult {
    // test with: (def! fibo ( lambda (n) (if (<= n 2) n (+ (fibo (- n 1)) (fibo (- n 2))))))
    let executable = env.borrow().lookup(name);

    match executable {
        Ok(RLType::Proc(func)) => {
            match make_atomic(tokens, env) {
                Ok(RLType::List(args)) => func(args),
                Ok(v) => error(format!("args are not a list: {}", v)),
                Err(e) => Err(e),
            }
        },
        Ok(RLType::Lambda(lambda)) => {
            match make_atomic(tokens, env) {
                Ok(RLType::List(ref mut args)) => eval_exec_lambda(lambda, args),
                Ok(v) => error(format!("args are not a list: {}", v)),
                Err(e) => Err(e),
            }
        },
        _ => error(format!("Not a function: {}", name))
    }
}

fn eval_exec_lambda(l: RLClosure, args: &mut Vec<RLType>) -> RLResult {
    if l.bindings.len() != args.len() {
        return error(format!("Invalid number of arguments for lambda: {}", args.len()));
    }

    // bind the args to the environment
    for i in 0..l.bindings.len() {
        l.env.borrow_mut().insert(l.bindings[i].clone(), args.remove(0));
    }

    // executes the lambda
    eval(*l.ast, &l.env)
}

// lambda keyword
// usage: lambda (params) (body)
// returns a closure, params should be symbols
fn eval_create_lambda( args: &mut Vec<RLType>, outer: &Env) -> RLResult {

    if args.len() > 2 || args.len() < 1 {
        return error(format!(
            "Invalid number of parameters for lambda: {}",
            args.len()))
    }

    let mut params: Vec<String> = Vec::new();

    // extract parameters if present
    if args.len() == 2 {
        if let RLType::List(values) = args.remove(0) {
            for value in values {
                match value {
                    RLType::Symbol(ref name) => params.push(name.to_owned()),
                    _ => return error(format!("Parameter is not a symbol: {}", value)),
                }
            }
        }
    }

    let lambda_env = Environment::new_with_outer(outer);
    let lambda = RLClosure {
        env: lambda_env,
        ast: Box::new(args.remove(0)),
        bindings: params
    };

    Ok(RLType::Lambda(lambda))
}

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
        error(format!("def!: key is not a symbol"))
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

// Implementation for list
// usage: (list v1 v2 ...)
fn eval_make_list(args: &Vec<RLType>, env: &Env) -> RLResult {
    match make_atomic(args.clone(), env) {
        Ok(RLType::List(elements)) => Ok(RLType::List(elements)),
        Ok(v) => error(format!("list eval error: {}", v)),
        Err(e) => Err(e),
    }
}

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

    match tree {
        Ok(cell) => eval(cell, env),
        Err(_) => Ok(RLType::Symbol("parse error.".to_string())),
    }
}

// evaluates a vector of values so that they are all atomic
fn make_atomic(tokens: Vec<RLType>, env: &Env) -> RLResult {
    let mut args: Vec<RLType> = Vec::new();
    for arg in tokens {
        match eval(arg, env) {
            Ok(value) => args.push(value),
            Err(e) => return Err(e),
        }
    }
    Ok(RLType::List(args))
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
