use std::collections::HashMap;

use super::token::Literal;
use super::stmt::Stmt;

pub struct Environment {
    funcs: HashMap<String, String>, // name and function data (includes a block stmt)
    types: HashMap<String, String>, // name and type data
    decls: HashMap<String, String>, // var name and decl data (type, mutability)
    stack: HashMap<String, Literal> // name and value
}

impl Environment {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
            types: HashMap::new(),
            decls: HashMap::new(),
            stack: HashMap::new()
        }
    }
    // TODO: add typing and hence declarations
    pub fn assign(&mut self, name: String, val: Literal) {
        self.stack.insert(name, val);
    }
    pub fn get(&self, name: String) -> /*Result<Literal, RuntimeError>*/ Option<&Literal> {
        self.stack.get(&name)
    }
}