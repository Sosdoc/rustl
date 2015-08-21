use std::collections::HashMap;
use lisp::cell::Cell;

pub struct Environment{
    pub map : HashMap<String, Cell>
}

impl Environment {

    pub fn new() -> Environment {
        Environment {
            map : HashMap::new()
        }
    }

    // Stub for default Environment
    pub fn default() -> Environment {
        let mut env = Environment::new();
        env.map.insert("pi".to_string(), Cell::Number(3.14));

        env.map.insert("+".to_string(), Cell::Proc(Environment::add));
        env.map.insert("-".to_string(), Cell::Proc(Environment::sub));
        env.map.insert("*".to_string(), Cell::Proc(Environment::mul));
        env.map.insert("/".to_string(), Cell::Proc(Environment::div));

        env
    }

    fn add(args : Cell) -> Cell {
        match args {
            Cell::List(v) => {
                let mut sum : f32 = 0.0;
                for arg in v {
                    sum += match arg {
                        Cell::Number(n) => n,
                        _ => panic!("Cannot add")
                    }
                }
                Cell::Number(sum)
            },
            _ => Cell::Nil
        }
    }

    fn sub(args : Cell) -> Cell {
        match args {
            Cell::List(mut v) => {
                let mut result : f32 = match v.remove(0) {
                    Cell::Number(n) => n,
                    _ => panic!("Cannot sub: not a number")
                };

                for arg in v {
                    result -= match arg {
                        Cell::Number(n) => n,
                        _ => panic!("Cannot sub: not a number")
                    }
                }

                Cell::Number(result)
            },
            _ => Cell::Nil
        }
    }

    pub fn mul(args : Cell) -> Cell {
        match args {
            Cell::List(v) => {
                let mut mul : f32 = 1.0;
                for arg in v {
                    mul *= match arg {
                        Cell::Number(n) => n,
                        _ => panic!("Cannot add")
                    }
                }
                Cell::Number(mul)
            },
            _ => Cell::Nil
        }
    }

    pub fn div(args : Cell) -> Cell {
        match args {
            Cell::List(mut v) => {
                let mut div : f32 = match v.remove(0) {
                    Cell::Number(n) => n,
                    _ => panic!("Cannot div: not a number")
                };

                for arg in v {
                    div /= match arg {
                        Cell::Number(n) if n != 0.0 => n,
                        _ => panic!("Cannot div: not a number")
                    }
                }

                Cell::Number(div)
            },
            _ => Cell::Nil
        }
    }
}
