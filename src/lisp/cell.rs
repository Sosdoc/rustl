use std::fmt::{Debug, Display, Formatter, Result};

// The Cell enum wraps all possible values in the language.
// It can be atomic (a string, function, number or one of the default values),
// or a list of other Cells, internally represented with a Vec
pub enum Cell {
    Symbol(String),
    Number(f32),
    Nil,
    True,
    False,
    Proc(fn(Cell) -> Cell),
    List(Vec<Cell>),
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Cell::Symbol(ref name) => write!(f, "s: {}", name),
            Cell::Number(num) => write!(f, "{}", num),
            Cell::List(ref tokens) => write!(f, "{:?}", tokens),
            Cell::Proc(_) => write!(f, "proc"),
            Cell::True => write!(f, "#t"),
            Cell::False => write!(f, "#f"),
            Cell::Nil => write!(f, "nil"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Cell::Symbol(ref name) => write!(f, "{}", name),
            Cell::Number(number) => write!(f, "{}", number),
            Cell::List(ref tokens) => write!(f, "{:?}", tokens),
            Cell::Proc(_) => write!(f, "proc"),
            Cell::True => write!(f, "#t"),
            Cell::False => write!(f, "#f"),
            Cell::Nil => write!(f, "nil"),
        }
    }
}


impl Cell {
    // Returns true if the Cell is atomic, i.e. not a List
    pub fn is_atom(&self) -> bool {
        match *self {
            Cell::List(_) => false,
            _ => true,
        }
    }
}
