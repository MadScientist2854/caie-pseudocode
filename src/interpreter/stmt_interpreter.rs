use crate::{stmt::Stmt, env::{Environment, Decl, Type}, token::Literal, expr::Expr};

impl super::Interpreter<()> for Stmt {
    fn interpret(&self, env: &mut Environment) {
        match self {
            Stmt::Block(stmts) => for stmt in stmts {
                stmt.interpret(env);
            },
            Stmt::ExprStmt(expr) => {expr.interpret(env);},
            Stmt::Declare(name, dtype) => env.declare(name.lexeme.clone(), Decl::new(true, dtype.clone())),
            Stmt::Constant(name, val) => {
                let val = val.interpret(env);
                env.declare(name.lexeme.clone(), Decl::new(false, Type::from_literal(&val)));
                env.assign(name.lexeme.clone(), val)
            },
            Stmt::Assign(name, val) => {
                let val = val.interpret(env);
                env.assign(name.lexeme.clone(), val)
            },
            Stmt::Input(expr) => if let Expr::IdentExpr(name) = expr.clone() {
                let mut val = String::new();
                std::io::stdin().read_line(&mut val).unwrap();
                env.assign(name.lexeme.clone(), Literal::String(val[0..val.len()-1].to_string())) // Cut off newline
            } else { panic!("expected identifier") },
            Stmt::Output(exprs) => { for expr in exprs {
                let val = expr.interpret(env);
                match val {
                    Literal::String(val) => print!("{}", val),
                    Literal::Char(val) => print!("{}", val),
                    _ => print!("{}", val.to_string())
                }
            } println!("")}, // print newline at the end
            Stmt::IfThen(cond, then_block, else_block) => {
                match cond.interpret(env) {
                    Literal::TRUE => then_block.interpret(env),
                    Literal::FALSE => match else_block {
                        Some(block) => block.interpret(env),
                        None => {}
                    },
                    _ => panic!("expected boolean expression")
                };
            },
        }
    }
}