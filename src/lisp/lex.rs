use lisp::cell::Cell;

// TODO: documentation

pub enum ParseError {
    UnbalancedParens,
    UnrecognizedToken,
    EOFReached,
}

pub type ParseResult = Result<Cell, ParseError>;

// Produces a vector of Strings, with no empty ones
pub fn tokenize(input: &str) -> Vec<String> {
    let formatted = format_braces(input);
    let mut tokens = Vec::new();

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

pub fn parse_form(tokens: &mut Vec<String>) -> ParseResult {
    match tokens[0].as_ref() {
        "(" => parse_list(tokens),
        ")" => Err(ParseError::UnbalancedParens),
        _ => parse_atom(tokens),
    }
}

fn parse_list(tokens: &mut Vec<String>) -> ParseResult {
    let mut list: Vec<Cell> = Vec::new();
    // discard first '(', this should not fail
    tokens.remove(0);

    while !tokens.is_empty() && &tokens[0] != ")" {
        match parse_form(tokens) {
            Ok(cell) => list.push(cell),
            Err(e) => return Err(e),
        }
    }
    
    if tokens.is_empty() {
        return Err(ParseError::UnbalancedParens);
    }
    
    tokens.remove(0);

    Ok(Cell::List(list))
}

fn parse_atom(tokens: &mut Vec<String>) -> ParseResult {
    let token = tokens.remove(0);

    match try_parse_number(&token) {
        Some(cell) => Ok(cell),
        None => Ok(parse_other_values(&token)),
    }
}

fn try_parse_number(text: &str) -> Option<Cell> {
    let n_f = text.parse::<f32>();

    match n_f {
        Ok(number) => Some(Cell::Number(number)),
        Err(_) => None,
    }
}

fn parse_other_values(text: &str) -> Cell {
    match text {
        "#t" => Cell::True,
        "#f" => Cell::False,
        "nil" => Cell::Nil,
        _ => Cell::Symbol(text.to_string()),
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
