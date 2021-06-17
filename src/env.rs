use std::collections::HashMap;

use super::token::Literal;
use super::stmt::Stmt;

pub struct Environment {
    procs: HashMap<String, Proc>,
    funcs: HashMap<String, Func>, // name and function data (includes a block stmt)
    types: HashMap<String, String>, // name and type data
    decls: HashMap<String, Decl>, // var name and decl data (type, mutability)
    stack: HashMap<String, Literal> // name and value
}

impl Environment {
    pub fn new() -> Self {
        Self {
            procs: HashMap::new(),
            funcs: HashMap::new(),
            types: HashMap::new(),
            decls: HashMap::new(),
            stack: HashMap::new()
        }
    }

    pub fn declare(&mut self, name: String, info: Decl)  -> Option<Decl> {
        self.decls.insert(name, info)
    }
    pub fn assign(&mut self, name: String, val: Literal) -> Option<Literal> /*-> Result<Option<Literal>, RuntimeError>*/ {
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
                    Literal::Date(_, _, _) => dtype == Type::Date,
                    Literal::Type(_) => todo!(),
                } { self.stack.insert(name, val) }
                else { panic!("Type of variable does not match with value") }
            } else { panic!("Cannot assign to constant") }
        } else {
            self.declare(name.clone(), Decl::new(true, Type::from_literal(&val)));
            self.stack.insert(name, val)
        }
    }
    pub fn get(&self, name: &String) -> /*Result<Literal, RuntimeError>*/ Option<&Literal> {
        self.stack.get(name)
        // if self.decls.contains_key(name) {
        //     Ok(self.stack.get(&name).unwrap())
        // } else { Err(RuntimeError::new("Uninitialized variable")) }
    }
    pub fn del(&mut self, name: &String) {
        self.decls.remove(name);
        self.stack.remove(name);
    }

    pub fn def_proc(&mut self, name: &String, arg_list: Vec<(String, Type)>, block: Stmt) {
        self.declare(name.clone(), Decl::new(false, Type::Proc));
        self.procs.insert(name.clone(), Proc::new(block, arg_list));
    }
    pub fn call_proc(&mut self, name: String, arg_list: Vec<Literal>) {
        //
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

    Proc,
    Func,
    Type
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
            Literal::Type(_) => todo!(),
        }
    }
}

pub struct Proc {
    block: Stmt,
    arg_list: Vec<(String, Type)>
}

impl Proc {
    pub fn new(block: Stmt, arg_list: Vec<(String, Type)>) -> Self { Self { block, arg_list } }
}

pub struct Func {
    block: Stmt,
    arg_list: Vec<(String, Type)>,
    ret_type: Type
}

#[derive(PartialEq)]
pub struct Decl {
    mutable: bool,
    dtype: Type
}

impl Decl {
    pub fn new(mutable: bool, dtype: Type) -> Self { Self { mutable, dtype } }
}