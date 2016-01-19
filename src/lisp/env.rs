use std::collections::HashMap;
use lisp::cell::Cell;

use lisp::modules::comparison;
use lisp::modules::math;

pub struct Environment<'a> {
    map: HashMap<String, Cell>,
    outer: Option<&'a Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Environment<'a> {
        Environment {
            map: HashMap::new(),
            outer: None
        }
    }

    pub fn new_with_outer(outer: &'a Environment) -> Environment<'a> {
        Environment {
            map: HashMap::new(),
            outer: Some(outer)
        }
    }

    // Stub for default Environment
    pub fn default() -> Environment<'a> {
        let mut env = Environment::new();

        env.map.insert("pi".to_string(), Cell::Number(3.14159265));
        // TODO: make modules a thing (i.e. Module trait so that the Environment adds it)
        math::add_module(&mut env);
        comparison::add_module(&mut env);

        env
    }

    // lookup searches in the current environment first, then tries in the outer environment if
    // available.
    pub fn lookup(&self, name: &String) -> Option<&Cell> {
        match self.map.get(name) {
            Some(c) => Some(c),
            None if self.outer.is_some() => self.outer.unwrap().lookup(name),
            _ => None
        }
    }

    // insert puts a value in the environment with the specified key (name)
    pub fn insert(&mut self, name: String, value: Cell) {
        self.map.insert(name, value);
    }
}
