use std::collections::HashMap;
use crate::interpreter::Interpreter;

use super::token::Literal;
use super::stmt::Stmt;

#[derive(Debug, Clone)]
pub struct Environment {
    parent_env: Option<Box<Environment>>,
    procs: HashMap<String, Proc>,
    funcs: HashMap<String, Func>, // name and function data (includes a block stmt)
    types: HashMap<String, String>, // name and type data
    decls: HashMap<String, Decl>, // var name and decl data (type, mutability)
    stack: HashMap<String, Literal>, // name and value
    ret: Option<Literal>
}

impl Environment {
    pub fn new(parent_env: Option<Box<Environment>>) -> Self {
        Self {
            parent_env,
            procs: HashMap::new(),
            funcs: HashMap::new(),
            types: HashMap::new(),
            decls: HashMap::new(),
            stack: HashMap::new(),
            ret: None
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
                if match val.clone() {
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
                    Literal::Type(_) => dtype == Type::Type,
                    Literal::Ref(lit, _) => dtype == Type::Ref(Box::new(Type::from_literal(lit.as_ref()))),
                } {
                    match val.clone() {
                        Literal::Ref(_, name) => self.parent_env.clone().unwrap().stack.insert(name, val),
                        _ => self.stack.insert(name, val)
                    }
                }
                else { panic!("Type of variable does not match with value") }
            } else { panic!("Cannot assign to constant") }
        } else {
            match &mut self.parent_env {
                Some(env) => match env.assign_parents(name.clone(), val.clone()) {
                    Some(lit) => Some(lit),
                    None => {
                        self.declare(name.clone(), Decl::new(true, Type::from_literal(&val)));
                        self.stack.insert(name, val)
                    }
                },
                None =>  {
                    self.declare(name.clone(), Decl::new(true, Type::from_literal(&val)));
                    self.stack.insert(name, val)
                }
            }
        }
    }
    fn assign_parents(&mut self, name: String, val: Literal) -> Option<Literal> /*-> Result<Option<Literal>, RuntimeError>*/ {
        if self.decls.contains_key(&name) {
            let decl = self.decls.get(&name).unwrap();
            if decl.mutable {
                let dtype = decl.dtype.clone();
                if match val.clone() {
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
                    Literal::Type(_) => dtype == Type::Type,
                    Literal::Ref(lit, _) => dtype == Type::Ref(Box::new(Type::from_literal(lit.as_ref()))),
                } { self.stack.insert(name, val) }
                else { panic!("Type of variable does not match with value") }
            } else { panic!("Cannot assign to constant") }
        } else {
            match &mut self.parent_env {
                Some(env) => env.assign(name, val),
                None => None
            }
        }
    }

    pub fn get_stack(&self, name: &String) -> /*Result<Literal, RuntimeError>*/ Literal {
        match self.decls.get(name) {
            Some(_) => match self.stack.get(name).unwrap() {
                Literal::Ref(lit, _) => *lit.clone(),
                lit => lit.clone()
            },
            None => match &self.parent_env {
                Some(env) => env.get_stack(name),
                None => panic!("reference to undefined variable")
            }
        }
    }
    pub fn get_proc(&self, name: &String) -> /*Result<Proc, RuntimeError>*/ Proc {
        match self.decls.get(name) {
            Some(_) => self.procs.get(name).expect(format!("{} is not a procedure", name).as_str()).clone(),
            None => match &self.parent_env {
                Some(env) => env.get_proc(name),
                None => panic!("reference to undefined variable")
            }
        }
    }
    pub fn get_func(&self, name: &String) -> /*Result<Func, RuntimeError>*/ Func {
        match self.decls.get(name) {
            Some(_) => self.funcs.get(name).unwrap().clone(),
            None => match &self.parent_env {
                Some(env) => env.get_func(name),
                None => panic!("reference to undefined variable")
            }
        }
    }

    pub fn del(&mut self, name: &String) {
        self.decls.remove(name);
        self.stack.remove(name);
    }

    pub fn def_proc(&mut self, name: &String, arg_list: Vec<(String, Type, bool)>, block: Stmt) {
        self.declare(name.clone(), Decl::new(false, Type::Proc));
        self.procs.insert(name.clone(), Proc::new(block, arg_list));
    }
    pub fn def_func(&mut self, name: &String, arg_list: Vec<(String, Type, bool)>, ret_type: Type, block: Stmt) {
        self.declare(name.clone(), Decl::new(false, Type::Func));
        self.funcs.insert(name.clone(), Func::new(block, arg_list, ret_type));
    }

    pub fn call_proc(&mut self, name: &String, arg_list: Vec<Literal>) {
        let proc = self.get_proc(name);
        let mut new_env = Environment::new(Some(Box::new(self.clone())));
        if proc.arg_list.len() != arg_list.len() { panic!("wrong number of arguments") }
        for i in 0..proc.arg_list.len() {
            if Type::from_literal(&arg_list[i]) == proc.arg_list[i].1 {
                if proc.arg_list[i].2 {
                    new_env.assign(proc.arg_list[i].0.clone(), Literal::Ref(Box::new(arg_list[i].clone()), proc.arg_list[i].0.clone()));
                } else { new_env.assign(proc.arg_list[i].0.clone(), arg_list[i].clone()); }
            } else { panic!("mismatched types of procedure argument") }
        }
        proc.run(&mut new_env);
    }
    pub fn call_func(&mut self, name: &String, arg_list: Vec<Literal>) -> Literal {
        let func = self.get_func(name);
        let mut new_env = Environment::new(Some(Box::new(self.clone())));
        if func.arg_list.len() != arg_list.len() { panic!("wrong number of arguments") }
        for i in 0..func.arg_list.len() {
            if Type::from_literal(&arg_list[i]) == func.arg_list[i].1 {
                if func.arg_list[i].2 {  }
                else { new_env.assign(func.arg_list[i].0.clone(), arg_list[i].clone()); }
            } else { panic!("mismatched types of function argument") }
        }
        func.run(&mut new_env)
    }

    pub fn set_ret(&mut self, val: Literal) {
        self.ret = Some(val);
    }
    pub fn reset_ret(&mut self) {
        self.ret = None;
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
    Type,
    Ref(Box<Type>)
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
            Literal::Ref(lit, _) => Type::Ref(Box::new(Type::from_literal(lit))),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Proc {
    block: Stmt,
    arg_list: Vec<(String, Type, bool)>
}

impl Proc {
    pub fn new(block: Stmt, arg_list: Vec<(String, Type, bool)>) -> Self { Self { block, arg_list } }

    pub fn run(&self, env: &mut Environment) {
        self.block.interpret(env);
    }
}

#[derive(Clone, Debug)]
pub struct Func {
    block: Stmt,
    arg_list: Vec<(String, Type, bool)>,
    ret_type: Type
}

impl Func {
    pub fn new(block: Stmt, arg_list: Vec<(String, Type, bool)>, ret_type: Type) -> Self
    { Self { block, arg_list, ret_type } }

    pub fn run(&self, env: &mut Environment) -> Literal {
        self.block.interpret(env);
        match &env.ret {
            Some(ret) => if !(Type::from_literal(ret) == self.ret_type)
                { panic!("mismatched type of return value") },
            None => panic!("expected RETURN statement"),
        }
        env.reset_ret();
        env.ret.clone().unwrap()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Decl {
    mutable: bool,
    dtype: Type
}

impl Decl {
    pub fn new(mutable: bool, dtype: Type) -> Self { Self { mutable, dtype } }
}