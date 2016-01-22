use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use lisp::types::*;

use lisp::modules::comparison;
use lisp::modules::math;

pub struct Environment {
    map: HashMap<String, RLType>,
    outer: Option<Env>,
}

pub type Env = Rc<RefCell<Environment>>;

impl Environment {
    pub fn new() -> Env {
        let env = Environment {
            map: HashMap::new(),
            outer: None
        };

        Rc::new(RefCell::new(env))
    }

    pub fn new_with_outer(outer: &Env) -> Env {
        let env = Environment {
            map: HashMap::new(),
            outer: Some(outer.clone())
        };

        Rc::new(RefCell::new(env))
    }

    // A default Environment with core functions
    pub fn default() -> Env {
        let mut env = Environment::new();

        // TODO: this should be in a separate file
        env.borrow_mut().insert("pi".to_string(), RLType::Number(3.14159265));
        math::add_module(&mut env);
        comparison::add_module(&mut env);

        env
    }

    // lookup searches in the current environment first, then tries in the outer environment if
    // available.
    // Values are currently cloned on retrieval... this could be costly, wrapping data types with
    // Rc pointers would be nice, but eval.rs has to be refactored to use immutable references.
    pub fn lookup(&self, name: &str) -> RLResult {
        match self.map.get(name) {
            Some(c) => Ok(c.clone()),
            None => {
                match self.outer {
                    Some(ref env) => env.borrow().lookup(name),
                    None => error(format!("No value for given key: {}", name)),
                }
            },
        }
    }

    // insert puts a value in the environment with the specified key (name)
    pub fn insert(&mut self, name: String, value: RLType) {
        self.map.insert(name, value);
    }
}
