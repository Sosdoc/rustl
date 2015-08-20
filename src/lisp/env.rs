#![allow(unused_variables)]
use std::collections::HashMap;

use lisp::lex::Token;

use lisp;

pub struct Environment{
    map : HashMap<String, Token>
}

impl Environment {

    pub fn new() -> Environment {
        Environment {
            map : HashMap::new()
        }
    }

    // Stub for default Environment
    pub fn default() -> Environment {
        let mut env = Environment::new();
        env.map.insert("pi".to_string(), Token::Number(3.14));

        env.map.insert("+".to_string(), Token::Proc(Environment::add));
        env.map.insert("-".to_string(), Token::Proc(Environment::sub));
        env.map.insert("*".to_string(), Token::Proc(Environment::mul));
        env.map.insert("/".to_string(), Token::Proc(Environment::div));

        env
    }

    pub fn add(args : Token) -> Token {
        match args {
            Token::List(v) => {
                let mut sum : f32 = 0.0;
                for arg in v {
                    sum += match arg {
                        Token::Number(n) => n,
                        _ => panic!("Cannot add")
                    }
                }
                Token::Number(sum)
            },
            _ => Token::Nil
        }
    }

    pub fn sub(args : Token) -> Token {
        match args {
            Token::List(mut v) => {
                let mut result : f32 = match v.remove(0) {
                    Token::Number(n) => n,
                    _ => panic!("Cannot sub: not a number")
                };

                for arg in v {
                    result -= match arg {
                        Token::Number(n) => n,
                        _ => panic!("Cannot sub: not a number")
                    }
                }

                Token::Number(result)
            },
            _ => Token::Nil
        }
    }

    pub fn mul(args : Token) -> Token {
        match args {
            Token::List(v) => {
                let mut mul : f32 = 1.0;
                for arg in v {
                    mul *= match arg {
                        Token::Number(n) => n,
                        _ => panic!("Cannot add")
                    }
                }
                Token::Number(mul)
            },
            _ => Token::Nil
        }
    }

    pub fn div(args : Token) -> Token {
        match args {
            Token::List(mut v) => {
                let mut div : f32 = match v.remove(0) {
                    Token::Number(n) => n,
                    _ => panic!("Cannot div: not a number")
                };

                for arg in v {
                    div /= match arg {
                        Token::Number(n) if n != 0.0 => n,
                        _ => panic!("Cannot div: not a number")
                    }
                }

                Token::Number(div)
            },
            _ => Token::Nil
        }
    }
}

pub fn eval(token : Token, env : &Environment) -> Token {

    match token {

        Token::Symbol(name) => {
            // check in the current env, return only values
            // missing Symbol?
            match env.map.get(&name).unwrap() {
                &Token::Number(n) => Token::Number(n),
                _ => Token::Nil
            }
        },
        Token::Number(n) => Token::Number(n),

        Token::List(mut tokens) => {
            // This branch does some heavy lifting
            // match on the first token then take action
            let first = tokens.remove(0);

            if let Token::Symbol(name) = first {

                if let &Token::Proc(f) = env.map.get(&name).unwrap() {
                    // eval each of the following tokens in the list
                    let mut args : Vec<Token> = Vec::new();

                    for arg in tokens {
                        args.push(eval(arg, env));
                    }
                    // executes the function with evaluated arguments
                    f(Token::List(args))

                } else {
                    // TODO: some keyword implementations here
                    let result : Token = match name.as_ref() {
                        "quote" => {
                            // returns the following token literally (should it be a symbol?)
                            tokens.remove(0)
                        },
                        "if" => {
                            Token::Nil
                        },
                        "set!" => {
                            Token::Nil
                        }

                        _ => Token::Nil
                    };

                    result
                }

            } else {
                // first is not a symbol, it's a regular list
                // TODO: should put back the first element
                Token::Nil
            }
        },


        _ => panic!("Unrecognized token") //Token::Nil
    }
}


// Utility function
fn parse_and_eval(input : &str) -> Token {
    let mut str_vec = lisp::lex::tokenize(input);
    let tokens = lisp::lex::parse_tree_from_tokens(&mut str_vec);
    lisp::env::eval(tokens.unwrap(), &lisp::env::Environment::default())
}


#[test]
fn eval_returns_pi() {
    let t = Token::Symbol("pi".to_string());
    let env = Environment::default();

    let res = match eval(t, &env) {
        Token::Number(n) => n,
        _ => 0.0
    };

    assert_eq!(res, 3.14);
}


#[test]
fn eval_sum() {
    let args = vec![Token::Symbol("+".to_string()),
                    Token::Number(2.0),
                    Token::Number(1.0)];

    let t = Token::List(args);
    let env = Environment::default();

    let res = match eval(t, &env) {
        Token::Number(n) => n,
        _ => 0.0
    };

    assert_eq!(res, 3.0);

    let args = vec![Token::Symbol("+".to_string()),
                    Token::Number(3.0),
                    Token::Number(2.0),
                    Token::Number(1.0)];

    let t = Token::List(args);
    let env = Environment::default();

    let res = match eval(t, &env) {
        Token::Number(n) => n,
        _ => 0.0
    };

    assert_eq!(res, 6.0);
}


#[test]
fn eval_sub() {
    let args = vec![Token::Symbol("-".to_string()),
                    Token::Number(3.0),
                    Token::Number(2.0),
                    Token::Number(1.0)];

    let t = Token::List(args);
    let env = Environment::default();

    let res = match eval(t, &env) {
        Token::Number(n) => n,
        _ => -10.0
    };

    assert_eq!(res, 0.0);
}
