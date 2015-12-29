use lisp::cell::Cell;

// TODO: documentation

pub enum ParseError {
    UnbalancedParens,
    UnrecognizedToken,
    EOFReached,
}

pub type ParseResult = Result<Cell, ParseError>;

// A Reader that provides a way to peek and read a stream of tokens.
pub struct Reader {
    tokens: Vec<String>,
    position: usize,
}

impl Reader {
    pub fn new(tokens: Vec<String>) -> Reader {
        Reader {
            tokens: tokens,
            position: 0,
        }
    }

    pub fn new_from_str(text: &str) -> Reader {
        Reader::new(tokenize(text))
    }

    pub fn peek(&self) -> Option<&str> {
        if self.position < self.tokens.len() {
            None
        } else {
            Some(self.tokens[self.position].as_ref())
        }
    }

    pub fn read(&mut self) -> Option<&str> {
        if self.position < self.tokens.len() {
            None
        } else {
            let next_token = self.tokens[self.position].as_ref();
            self.position += 1;
            Some(next_token)
        }
    }
}

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

pub fn parse_form(reader: &Reader) -> ParseResult {
    if let Some(token) = reader.peek() {
        match token {
            "(" => parse_list(reader),
            _ => parse_atom(reader),
        }
    }
}

pub fn parse_list(reader: &Reader) -> ParseResult {
    // TODO: check for unbalanced parens (review loop)
    let mut list: Vec<Cell> = Vec::new();
    // discard first '('
    let _ = reader.read();

    while let Some(token) = reader.read() {
        if token != ")" {
            match parse_form(reader) {
                Ok(cell) => list.push(cell),
                Err(e) => return Err(e), 
            }
        } else {
            break;
        }
    }

    Ok(Cell::List(list))
}

fn parse_atom(reader: &Reader) -> ParseResult {
    match reader.read() {
        Some(token) => {
            match try_parse_number(token) {
                Some(cell) => Ok(cell),
                None => Ok(parse_other_values(token)),
            }
        }
        None => Err(ParseError::EOFReached),
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

// Parses an atomic value.
// It first attempts to parse a number (only float for now), then matches against other atomic
// types.
// fn parse_atom(input: &str) -> Cell {
//     let n_f = input.parse::<f32>();

//     if n_f.is_err() {
//         match input {
//             "#t" => Cell::True,
//             "#f" => Cell::False,
//             "nil" => Cell::Nil,
//             _ => Cell::Symbol(input.to_string()),
//         }
//     } else {
//         Cell::Number(n_f.unwrap())
//     }
// }


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
