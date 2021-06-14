mod stmt_interpreter;
mod expr_interpreter;

use super::env::Environment;

pub trait Interpreter<Return> {
    fn interpret(&self, env: &mut Environment) -> Return;
}