use std::fmt::{Debug, Formatter, Result};

pub enum Token {
    // possible atomic tokens: symbols or numbers (either ints or floats)
    Symbol(String),
    Number(f32),
    Proc(fn(Token) -> Token),
    // TODO: should this be removed in favor of Option/env Token?
    Nil,
    True,
    False,
    // lists are vectors of tokens
    List(Vec<Token>)
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &Token::Symbol(ref name) => write!(f, "s: {}", name),
            &Token::Number(n) => write!(f, "{}", n),
            &Token::List(ref tokens) => write!(f, "{:?}", tokens),
            &Token::Proc(_) => write!(f, "proc"),
            &Token::True => write!(f, "#t"),
            &Token::False => write!(f, "#f"),
            _ => write!(f, "unknown")
        }
    }
}

// Produces a vector of Strings, with no empty ones
pub fn tokenize(input : &str) -> Vec<String> {
    let formatted = format_braces(input);
    let mut tokens : Vec<String> = Vec::new();

    for token in formatted.split(' ') {
        let t = token.trim();
        // push only tokens with text
        if t != "" {
            tokens.push(t.to_string());
        }
    }

    tokens
}

// Puts spaces befor and after braces
fn format_braces(input : &str) -> String {
    input.replace("(", " ( ").replace(")", " ) ").trim().to_string()
}

// Consumes a vector of Strings, returning data to be used with eval
pub fn parse_tree_from_tokens(tokens : &mut Vec<String>) -> Option<Token> {
    let mut result : Option<Token> = None;

    if !tokens.is_empty() {
        let curr_token = tokens.remove(0);

        match curr_token.as_ref() {
            "(" => {
                    // build a list
                    let mut l : Vec<Token> = Vec::new();

                    while &tokens[0] != ")" {
                        l.push(parse_tree_from_tokens(tokens).unwrap());
                    }

                    tokens.remove(0);
                    result = Some(Token::List(l));
                },
            ")" => panic!("Unbalanced )"),
            s => {
                // parse the single atom
                result = Some(parse_atom(s));
            }
        }

    }

    result
}

// Parses an atomic value (number/string)
fn parse_atom(input : &str) -> Token {
    // try parsing as i32
    let n_int = input.parse::<i32>();
    if n_int.is_err() {
        // try parsing as f32
        let n_f = input.parse::<f32>();
        if n_f.is_err() {
            return Token::Symbol(input.to_string());
        }
        return Token::Number(n_f.unwrap());
    }
    Token::Number(n_int.unwrap() as f32)
}


#[test]
fn test_parse_atom() {

    let atom = "123";
    let token = match parse_atom(atom) {
        Token::Number(n) => n,
        _ => 0.0
    };

    assert_eq!(token, 123.0);

    let atom = "12.3";
    let token = match parse_atom(atom) {
        Token::Number(n) => n,
        _ => 0.0
    };

    assert_eq!(token, 12.3);

    let atom = "12g.3";
    let token = match parse_atom(atom) {
        Token::Symbol(n) => n,
        _ => "wat".to_string()
    };

    assert_eq!(&token, atom);
}
