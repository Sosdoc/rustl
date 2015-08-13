#![allow(unused_variables)]

use std::ops::{Add, Sub, Mul, Div};
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
        env.map.insert("pi".to_string(), Token::Float(3.14));
        env
    }

    pub fn add<T>(a: T, b: T) -> T::Output
        where T: Add {
        a + b
    }

    fn sub<T>(a: T, b: T) -> T::Output
        where T: Sub {
        a - b
    }

    fn mul<T>(a: T, b: T) -> T::Output
        where T: Mul {
        a * b
    }

    fn div<T>(a: T, b: T) -> T::Output
        where T: Div {
        a / b
    }
}

pub fn eval(token : Token, env : Environment) -> Token {

    match token {

        Token::Symbol(name) => {
            // check in the current env, return only values
            // missing Symbol?
            match env.map.get(&name).unwrap() {
                &Token::Float(n) => Token::Float(n),
                &Token::Number(n) => Token::Number(n),
                _ => Token::Nil
            }
        },
        // better way of doing this?
        Token::Number(n) => Token::Number(n),
        Token::Float(f) => Token::Float(f),

        Token::List(mut tokens) => {
            // This branch does the heavy lifting
            // match on the first token then take action
            let first = tokens.remove(0);

            if let Token::Symbol(name) = first {

                let result : Token = match name.as_ref() {
                    "quote" => {
                        tokens.remove(0)
                    },
                    "if" => {
                        Token::Nil
                    }

                    _ => Token::Nil
                };

                // put result in a token

            } else {
                // not a symbol?
            }

            Token::Nil

        },


        _ => panic!("Unrecognized token") //Token::Nil
    }
}
