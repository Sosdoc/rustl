// This module contains comparison functions for numeric values

use lisp::env::Environment;
use lisp::cell::Cell;


// Adds this module's functions to the provided environment
pub fn add_module(env: &mut Environment) {
    env.insert("+".to_string(), Cell::Proc(add));
    env.insert("-".to_string(), Cell::Proc(sub));
    env.insert("*".to_string(), Cell::Proc(mul));
    env.insert("/".to_string(), Cell::Proc(div));
}

fn add(args: Cell) -> Cell {
    match args {
        Cell::List(v) => {
            let mut sum: f32 = 0.0;
            for arg in v {
                sum += match arg {
                    Cell::Number(n) => n,
                    _ => panic!("Cannot add"),
                }
            }
            Cell::Number(sum)
        }
        _ => Cell::Nil,
    }
}

fn sub(args: Cell) -> Cell {
    match args {
        Cell::List(mut v) => {
            let mut result: f32 = match v.remove(0) {
                Cell::Number(n) => n,
                _ => panic!("Cannot sub: not a number"),
            };

            for arg in v {
                result -= match arg {
                    Cell::Number(n) => n,
                    _ => panic!("Cannot sub: not a number"),
                }
            }

            Cell::Number(result)
        }
        _ => Cell::Nil,
    }
}

fn mul(args: Cell) -> Cell {
    match args {
        Cell::List(v) => {
            let mut mul: f32 = 1.0;
            for arg in v {
                mul *= match arg {
                    Cell::Number(n) => n,
                    _ => panic!("Cannot add"),
                }
            }
            Cell::Number(mul)
        }
        _ => Cell::Nil,
    }
}

fn div(args: Cell) -> Cell {
    match args {
        Cell::List(mut v) => {
            let mut div: f32 = match v.remove(0) {
                Cell::Number(n) => n,
                _ => panic!("Cannot div: not a number"),
            };

            for arg in v {
                div /= match arg {
                    Cell::Number(n) if n != 0.0 => n,
                    _ => panic!("Cannot div: not a number"),
                }
            }

            Cell::Number(div)
        }
        _ => Cell::Nil,
    }
}
