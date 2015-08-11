#[derive(Debug)]
pub enum Token {
    // possible atomic tokens: symbols or numbers (either ints or floats)
    Symbol(String),
    Number(i32),
    Float(f32),
    // lists are groups of tokens
    List(Vec<Token>)
}


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

fn format_braces(input : &str) -> String {
    input.replace("(", " ( ").replace(")", " ) ").trim().to_string()
}

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
                result = parse_atom(s);
            }
        }

    }

    result
}

fn parse_atom(input : &str) -> Option<Token> {
    // try parsing as i32
    let n_int = input.parse::<i32>();

    if n_int.is_err() {
        // try parsing as f32
        let n_f = input.parse::<f32>();

        if n_f.is_err() {
            // fuck this shit, return the fucking token
            return Some(Token::Symbol(input.to_string()));
        }
        return Some(Token::Float(n_f.unwrap()));
    }
    Some(Token::Number(n_int.unwrap()))
}


#[test]
fn test_parse_atom() {

    let atom = "123";
    let token = match parse_atom(atom) {
        Some(Token::Number(n)) => n,
        _ => 0
    };

    assert_eq!(token, 123);

    let atom = "12.3";
    let token = match parse_atom(atom) {
        Some(Token::Float(n)) => n,
        _ => 0.0
    };

    assert_eq!(token, 12.3);

    let atom = "12g.3";
    let token = match parse_atom(atom) {
        Some(Token::Symbol(n)) => n,
        _ => "wat".to_string()
    };

    assert_eq!(&token, atom);

}
