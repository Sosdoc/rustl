use std::fmt::{Debug, Display, Formatter, Result};

// The Cell enum wraps all possible values in the language.
// It can be atomic (a string, function, number or one of the default values),
// or a list of other Cells, internally represented with a Vec
pub enum Cell {
    Symbol(String),
    Number(f32),
    Proc(fn(Cell) -> Cell),
    Nil,
    True,
    False,
    List(Vec<Cell>)
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Cell::Symbol(ref name) => write!(f, "s: {}", name),
            Cell::Number(n) => write!(f, "{}", n),
            Cell::List(ref tokens) => write!(f, "{:?}", tokens),
            Cell::Proc(_) => write!(f, "proc"),
            Cell::True => write!(f, "#t"),
            Cell::False => write!(f, "#f"),
            _ => write!(f, "unknown")
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Cell::Symbol(ref name) => write!(f, "{}", name),
            Cell::Number(n) => write!(f, "{:3}", n),
            Cell::List(ref tokens) => write!(f, "{:?}", tokens),
            Cell::Proc(_) => write!(f, "proc"),
            Cell::True => write!(f, "#t"),
            Cell::False => write!(f, "#f"),
            _ => write!(f, "unknown")
        }
    }
}


impl Cell {

    // Returns true if the Cell is atomic, i.e. not a List
    pub fn is_atom(&self) -> bool {
        match *self {
            Cell::List(_) => false,
            _ => true
        }
    }
}
