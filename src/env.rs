use std::collections::HashMap;

use super::token::Literal;
use super::stmt::Stmt;

pub struct Environment {
    funcs: HashMap<String, Func>, // name and function data (includes a block stmt)
    types: HashMap<String, String>, // name and type data
    decls: HashMap<String, Decl>, // var name and decl data (type, mutability)
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
    pub fn declare(&mut self, name: String, info: Decl) {
        self.decls.insert(name, info);
    }
    pub fn assign(&mut self, name: String, val: Literal) /*-> Result<(), RuntimeError>*/ {
        if self.decls.contains_key(&name) {
            let decl = self.decls.get(&name).unwrap();
            if decl.mutable {
                let dtype = decl.dtype.clone();
                if match val {
                    Literal::TRUE => dtype == Type::Bool,
                    Literal::FALSE => dtype == Type::Bool,
                    Literal::READ => todo!(),
                    Literal::WRITE => todo!(),
                    Literal::APPEND => todo!(),
                    Literal::RANDOM => todo!(),
                    Literal::Int(_) => dtype == Type::Int,
                    Literal::Float(_) => dtype == Type::Float,
                    Literal::Char(_) => dtype == Type::Char,
                    Literal::String(_) => dtype == Type::String,
                    Literal::Date(_, _, _) => dtype == Type::Date
                } { self.stack.insert(name, val); }
                else { panic!("Type of variable does not match with value") }
            } else { panic!("Cannot assign to constant") }
        } else {
            self.declare(name.clone(), Decl::new(false, Type::from_literal(&val)));
            self.stack.insert(name, val);
        }
    }
    pub fn get(&self, name: String) -> /*Result<Literal, RuntimeError>*/ Option<&Literal> {
        self.stack.get(&name)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    // FileType,
    Bool,
    Int,
    Float,
    Char,
    String,
    Date,
    //UDT(UDT)
}

impl Type {
    pub fn from_literal(lit: &Literal) -> Type {
        match lit {
            Literal::TRUE | Literal::FALSE => Type::Bool,
            Literal::READ => todo!(),
            Literal::WRITE => todo!(),
            Literal::APPEND => todo!(),
            Literal::RANDOM => todo!(),
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::Char(_) => Type::Char,
            Literal::String(_) => Type::String,
            Literal::Date(_, _, _) => Type::Date,
        }
    }
}

pub struct Func {
    block: Stmt,
    arg_list: HashMap<String, Type>,
    retdtype: Option<Type>
}

#[derive(PartialEq)]
pub struct Decl {
    mutable: bool,
    dtype: Type
}

impl Decl {
    pub fn new(mutable: bool, dtype: Type) -> Self { Self { mutable, dtype } }
}