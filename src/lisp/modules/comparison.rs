// This module contains comparison functions for numeric values

use lisp::env::Environment;
use lisp::cell::Cell;

// Adds this module's functions to the provided environment
pub fn add_module(env: &mut Environment) {
    env.insert(">".to_string(), Cell::Proc(gt));
    env.insert(">=".to_string(), Cell::Proc(gte));
    env.insert("<".to_string(), Cell::Proc(lt));
    env.insert("<=".to_string(), Cell::Proc(lte));
    env.insert("=".to_string(), Cell::Proc(eq));
}

fn gt(args: Cell) -> Cell {
    match extract_two_numbers(args) {
        Some((left, right)) => {
            if left > right {
                Cell::True
            } else {
                Cell::False
            }
        }
        _ => Cell::Nil,
    }
}

fn gte(args: Cell) -> Cell {
    match extract_two_numbers(args) {
        Some((left, right)) => {
            if left >= right {
                Cell::True
            } else {
                Cell::False
            }
        }
        _ => Cell::Nil,
    }
}

fn lt(args: Cell) -> Cell {
    match extract_two_numbers(args) {
        Some((left, right)) => {
            if left < right {
                Cell::True
            } else {
                Cell::False
            }
        }
        _ => Cell::Nil,
    }
}

fn lte(args: Cell) -> Cell {
    match extract_two_numbers(args) {
        Some((left, right)) => {
            if left <= right {
                Cell::True
            } else {
                Cell::False
            }
        }
        _ => Cell::Nil,
    }
}

fn eq(args: Cell) -> Cell {
    match extract_two_numbers(args) {
        Some((left, right)) => {
            if left == right {
                Cell::True
            } else {
                Cell::False
            }
        }
        _ => Cell::Nil,
    }
}


fn extract_two_numbers(args: Cell) -> Option<(f32, f32)> {
    match args {
        Cell::List(args) => {
            let mut numbers = vec![];

            if args.len() == 2 {
                for cell in args {
                    match cell {
                        Cell::Number(n) => numbers.push(n),
                        _ => panic!("Cannot compare {}", cell),
                    }
                }
                Some((numbers[0], numbers[1]))
            } else {
                None
            }
        }
        _ => None,
    }
}
