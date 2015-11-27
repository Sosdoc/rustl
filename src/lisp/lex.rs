use lisp::cell::Cell;

// Produces a vector of Strings, with no empty ones
pub fn tokenize(input: &str) -> Vec<String> {
    let formatted = format_braces(input);
    let mut tokens: Vec<String> = Vec::new();

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
fn format_braces(input: &str) -> String {
    input.replace("(", " ( ").replace(")", " ) ").trim().to_string()
}

// Consumes a vector of Strings, returning data to be used with eval
pub fn parse_tree_from_tokens(tokens: &mut Vec<String>) -> Option<Cell> {
    let mut result: Option<Cell> = None;

    if !tokens.is_empty() {
        let curr_token = tokens.remove(0);

        match curr_token.as_ref() {
            "(" => {
                // build a list
                let mut l: Vec<Cell> = Vec::new();

                while &tokens[0] != ")" {
                    l.push(parse_tree_from_tokens(tokens).unwrap());
                }

                tokens.remove(0);
                result = Some(Cell::List(l));
            }
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
fn parse_atom(input: &str) -> Cell {
    let n_f = input.parse::<f32>();

    if n_f.is_err() {
        match input {
            "#t" => Cell::True,
            "#f" => Cell::False,
            "nil" => Cell::Nil,
            _ => Cell::Symbol(input.to_string()),
        }
    } else {
        Cell::Number(n_f.unwrap())
    }
}


#[test]
fn test_parse_atom() {
    let atom = "123";
    let token = match parse_atom(atom) {
        Cell::Number(n) => n,
        _ => 0.0,
    };

    assert_eq!(token, 123.0);

    let atom = "12.3";
    let token = match parse_atom(atom) {
        Cell::Number(n) => n,
        _ => 0.0,
    };

    assert_eq!(token, 12.3);

    let atom = "12g.3";
    let token = match parse_atom(atom) {
        Cell::Symbol(n) => n,
        _ => "wat".to_string(),
    };

    assert_eq!(&token, atom);
}
