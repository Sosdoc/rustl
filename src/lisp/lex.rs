use lisp::types::*;

// TODO: documentation

pub enum ParseError {
    UnbalancedParens,
    EOFReached,
}

pub type ParseResult = Result<RLType, ParseError>;

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
    if tokens.is_empty() {
        return Err(ParseError::EOFReached);
    }

    match tokens[0].as_ref() {
        "(" => parse_list(tokens),
        ")" => Err(ParseError::UnbalancedParens),
        _ => parse_atom(tokens),
    }
}

fn parse_list(tokens: &mut Vec<String>) -> ParseResult {
    let mut list: Vec<RLType> = Vec::new();
    // discard first '(', this fails of course
    tokens.remove(0);

    while !tokens.is_empty() && &tokens[0] != ")" {
        let cell = try!(parse_form(tokens));
        list.push(cell);
    }

    if tokens.is_empty() {
        return Err(ParseError::UnbalancedParens);
    }

    tokens.remove(0);
    Ok(RLType::List(list))
}

fn parse_atom(tokens: &mut Vec<String>) -> ParseResult {
    let token = tokens.remove(0);

    match parse_number(&token) {
        Some(cell) => Ok(cell),
        None => Ok(parse_other_values(&token)),
    }
}

fn parse_number(text: &str) -> Option<RLType> {
    let n_f = text.parse::<f32>();

    match n_f {
        Ok(number) => Some(RLType::Number(number)),
        Err(_) => None,
    }
}

fn parse_other_values(text: &str) -> RLType {
    match text {
        "#t" => RLType::True,
        "#f" => RLType::False,
        "nil" => RLType::Nil,
        _ => RLType::Symbol(text.to_string()),
    }
}

// #[test]
// fn test_parse_atom() {
//     let atom = vec!["123"];
//     let token = match try!(parse_atom(atom)) {
//         RLType::Number(n) => n,
//         _ => 0.0,
//     };
//
//     assert_eq!(token, 123.0);
//
//     let atom = "12.3";
//     let token = match parse_atom(atom) {
//         RLType::Number(n) => n,
//         _ => 0.0,
//     };
//
//     assert_eq!(token, 12.3);
//
//     let atom = "12g.3";
//     let token = match parse_atom(atom) {
//         RLType::Symbol(n) => n,
//         _ => "wat".to_string(),
//     };
//
//     assert_eq!(&token, atom);
// }
