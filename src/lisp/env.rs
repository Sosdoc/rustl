use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use lisp::types::*;

use lisp::modules::comparison;
use lisp::modules::math;


pub struct Binding {
    pub key: String,
    pub expr: RLType,
}

#[derive(Clone)]
pub struct Environment {
    map: HashMap<String, RLType>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            map: HashMap::new(),
            outer: None
        }
    }

    pub fn new_with_outer(outer: Environment) -> Environment {
        Environment {
            map: HashMap::new(),
            outer: Some(Rc::new(RefCell::new(outer)))
        }
    }

    pub fn new_with_bindings(outer: Environment, binds: Vec<Binding>) -> Environment {
        let mut env = Environment::new_with_outer(outer);

        for binding in binds {
            env.insert(binding.key, binding.expr)
        }

        env
    }

    // Stub for default Environment
    pub fn default() -> Environment {
        let mut env = Environment::new();

        // TODO: this should be in a separate file
        env.map.insert("pi".to_string(), RLType::Number(3.14159265));
        math::add_module(&mut env);
        comparison::add_module(&mut env);

        env
    }

    // lookup searches in the current environment first, then tries in the outer environment if
    // available.
    pub fn lookup(&self, name: &String) -> RLResult {
        match self.map.get(name) {
            Some(c) => Ok(c.clone()),
            None => {
                match self.outer {
                    Some(ref env) => env.borrow().lookup(name),
                    None => error("No value for given key"),
                }
            },
        }
    }

    // insert puts a value in the environment with the specified key (name)
    pub fn insert(&mut self, name: String, value: RLType) {
        self.map.insert(name, value);
    }
}
