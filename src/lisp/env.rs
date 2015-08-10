use std::ops::{Add, Sub, Mul, Div};
use std::collections::HashMap;

pub enum Cell {
    Symbol(String),
    Number(f32),
    List(Vec<Cell>),
    Proc,
    Lambda
}

pub struct Env {
    map : HashMap<String, Cell>
}

impl Env {
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
