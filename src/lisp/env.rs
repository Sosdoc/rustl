#![allow(unused_variables)]
use std::collections::HashMap;

use lisp::lex::Token;

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
            Token::List(v) => {
                let mut sum : f32 = 0.0;
                for arg in v {
                    sum -= match arg {
                        Token::Number(n) => n,
                        _ => panic!("Cannot sub")
                    }
                }
                Token::Number(sum)
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
                    _ => 0.0
                };

                for arg in v {
                    div /= match arg {
                        Token::Number(n) if n != 0.0 => n,
                        _ => panic!("Cannot even div")
                    }
                }

                Token::Number(div)
            },
            _ => Token::Nil
        }
    }
}

pub fn eval(token : Token, env : Environment) -> Token {

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
                    // executes functions stored in the environment
                    f(Token::List(tokens))
                } else {
                    // some keyword implementations here
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

                    // put result in a token
                    result
                }



            } else {
                // first is not a symbol, it's a regular list
                // should put back the first element
                Token::Nil
            }
        },


        _ => panic!("Unrecognized token") //Token::Nil
    }
}


#[test]
fn eval_returns_pi() {
    let t = Token::Symbol("pi".to_string());
    let env = Environment::default();

    let res = match eval(t, env) {
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

    let res = match eval(t, env) {
        Token::Number(n) => n,
        _ => 0.0
    };

    assert_eq!(res, 3.0);
}

#[test]
fn eval_sum_multiple() {
    let args = vec![Token::Symbol("+".to_string()),
                    Token::Number(3.0),
                    Token::Number(2.0),
                    Token::Number(1.0)];

    let t = Token::List(args);
    let env = Environment::default();

    let res = match eval(t, env) {
        Token::Number(n) => n,
        _ => 0.0
    };

    assert_eq!(res, 6.0);
}
