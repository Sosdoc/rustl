use std::collections::HashMap;
use lisp::cell::Cell;

use lisp::modules::comparison;
use lisp::modules::math;

pub struct Environment {
    pub map: HashMap<String, Cell>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment { map: HashMap::new() }
    }

    // Stub for default Environment
    pub fn default() -> Environment {
        let mut env = Environment::new();

        env.map.insert("pi".to_string(), Cell::Number(3.14159265));
        // TODO: make modules a thing (i.e. Module trait so that the Environment adds it)
        math::add_module(&mut env);
        comparison::add_module(&mut env);

        env
    }
}
