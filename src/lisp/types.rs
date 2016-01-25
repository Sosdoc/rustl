use std;
use std::fmt::{Debug, Display, Formatter};

// The RLType (RustLisp) enum wraps all possible values in the language.
// It can be atomic (a string, function, number or one of the default values),
// or a list of other RLTypes, internally represented with a Vec
#[derive(Clone)]
pub enum RLType {
    Nil,
    True,
    False,
    Symbol(String),
    Number(f32),
    Proc(fn(Vec<RLType>) -> RLResult),
    Lambda(RLClosure),
    List(Vec<RLType>),
}

#[derive(Clone)]
pub struct RLClosure {
    pub ast: Box<RLType>,
    pub bindings: Vec<String>,
}

pub enum RLError {
    Message(String),
    InvalidValue(RLType),
}

impl RLError {
    pub fn get_message(&self) -> String {
        match *self {
            RLError::Message(ref s) => s.to_string(),
            RLError::InvalidValue(ref v) => format!("{}", v),
        }
    }
}

pub type RLResult = Result<RLType, RLError>;

pub fn error(message: String) -> RLResult {
    Err(RLError::Message(message.to_string()))
}

impl Debug for RLType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            RLType::Symbol(ref name) => write!(f, "\"{}\"", name),
            RLType::Number(num) => write!(f, "{}", num),
            RLType::List(ref tokens) => write!(f, "{:?}", tokens),
            RLType::Proc(_) => write!(f, "proc"),
            RLType::Lambda(_) => write!(f, "lambda"),
            RLType::True => write!(f, "#t"),
            RLType::False => write!(f, "#f"),
            RLType::Nil => write!(f, "nil"),
        }
    }
}

// TODO: display/debug should show different things ideally
impl Display for RLType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            RLType::Symbol(ref name) => write!(f, "\"{}\"", name),
            RLType::Number(number) => write!(f, "{}", number),
            RLType::List(ref tokens) => write!(f, "{:?}", tokens),
            RLType::Proc(_) => write!(f, "proc"),
            RLType::Lambda(_) => write!(f, "lambda"),
            RLType::True => write!(f, "#t"),
            RLType::False => write!(f, "#f"),
            RLType::Nil => write!(f, "nil"),
        }
    }
}


impl RLType {
    // Returns true if the RLType is atomic, i.e. not a List
    pub fn is_atom(&self) -> bool {
        match *self {
            RLType::List(_) => false,
            _ => true,
        }
    }
}
