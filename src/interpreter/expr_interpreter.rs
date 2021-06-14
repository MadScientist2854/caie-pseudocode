use crate::{token::{Literal, TokenType}, expr::Expr, env::Environment};

impl super::Interpreter<Literal> for Expr {
    fn interpret(&self, env: &mut Environment) -> Literal {
        match self {
            Expr::Unary(op, right) => match op.ttype {
                TokenType::NOT => match right.interpret(env) {
                    Literal::TRUE => Literal::FALSE,
                    Literal::FALSE => Literal::TRUE,
                    _ => panic!("expected boolean expression")
                },
                TokenType::Minus => match right.interpret(env) {
                    Literal::Int(val) => Literal::Int(-val),
                    Literal::Float(val) => Literal::Float(-val),
                    _ => panic!("expected boolean expression")
                },
                _ => panic!("invalid syntax tree (unary operator)")
            },
            Expr::Binary(left, op, right) => match op.ttype {
                TokenType::Equal => if left.interpret(env) == right.interpret(env) { Literal::TRUE }
                    else { Literal::FALSE },
                TokenType::NotEqual => if left.interpret(env) == right.interpret(env) { Literal::FALSE }
                else { Literal::TRUE },
                // TokenType::Period => todo!(),
                TokenType::Star => {
                    let left = left.interpret(env);
                    let right = right.interpret(env);
                    match left {
                        // Literal::Char(_) => todo!(),
                        // Literal::String(_) => todo!(),
                        // Literal::Date(_, _, _) => todo!(),
                        Literal::Int(left_i) => match right {
                            // Literal::Char(_) => todo!(),
                            // Literal::String(_) => todo!(),
                            // Literal::Date(_, _, _) => todo!(),
                            Literal::Int(right_i) => Literal::Int(left_i * right_i),
                            Literal::Float(right_f) => Literal::Float(left_i as f32 * right_f),
                            _ => panic!("expected numerical value")
                        },
                        Literal::Float(left_f) => match right {
                            // Literal::Char(_) => todo!(),
                            // Literal::String(_) => todo!(),
                            // Literal::Date(_, _, _) => todo!(),
                            Literal::Int(right_i) => Literal::Float(left_f * right_i as f32),
                            Literal::Float(right_f) => Literal::Float(left_f * right_f),
                            _ => panic!("expected numerical value")
                        },
                        _ => panic!("expected numerical value")
                    }
                },
                TokenType::Slash => {
                    let left = left.interpret(env);
                    let right = right.interpret(env);
                    match left {
                        Literal::Int(left_i) => match right {
                            Literal::Int(right_i) => Literal::Int(left_i / right_i),
                            Literal::Float(right_f) => Literal::Float(left_i as f32 / right_f),
                            _ => panic!("expected numerical value")
                        },
                        Literal::Float(left_f) => match right {
                            Literal::Int(right_i) => Literal::Float(left_f / right_i as f32),
                            Literal::Float(right_f) => Literal::Float(left_f / right_f),
                            _ => panic!("expected numerical value")
                        },
                        _ => panic!("expected numerical value")
                    }
                },
                TokenType::Plus => {
                    let left = left.interpret(env);
                    let right = right.interpret(env);
                    match left {
                        // Literal::Date(_, _, _) => todo!(),
                        Literal::Int(left_i) => match right {
                            // Literal::Date(_, _, _) => todo!(),
                            Literal::Int(right_i) => Literal::Int(left_i + right_i),
                            Literal::Float(right_f) => Literal::Float(left_i as f32 + right_f),
                            _ => panic!("expected numerical value")
                        },
                        Literal::Float(left_f) => match right {
                            // Literal::Date(_, _, _) => todo!(),
                            Literal::Int(right_i) => Literal::Float(left_f + right_i as f32),
                            Literal::Float(right_f) => Literal::Float(left_f + right_f),
                            _ => panic!("expected numerical value")
                        },
                        _ => panic!("expected numerical value")
                    }
                },
                TokenType::Minus => {
                    let left = left.interpret(env);
                    let right = right.interpret(env);
                    match left {
                        // Literal::Date(_, _, _) => todo!(),
                        Literal::Int(left_i) => match right {
                            // Literal::Date(_, _, _) => todo!(),
                            Literal::Int(right_i) => Literal::Int(left_i - right_i),
                            Literal::Float(right_f) => Literal::Float(left_i as f32 - right_f),
                            _ => panic!("expected numerical value")
                        },
                        Literal::Float(left_f) => match right {
                            // Literal::Date(_, _, _) => todo!(),
                            Literal::Int(right_i) => Literal::Float(left_f - right_i as f32),
                            Literal::Float(right_f) => Literal::Float(left_f - right_f),
                            _ => panic!("expected numerical value")
                        },
                        _ => panic!("expected numerical value")
                    }
                },
                TokenType::Less => {
                    let left = left.interpret(env); //TODO
                    let right = right.interpret(env);
                    match left {
                        // Literal::Char(_) => todo!(), Alphabetical order
                        // Literal::String(_) => todo!(), for char and string
                        // Literal::Date(_, _, _) => todo!(),
                        Literal::Int(left_i) => match right {
                            // Literal::Char(_) => todo!(),
                            // Literal::String(_) => todo!(),
                            // Literal::Date(_, _, _) => todo!(),
                            Literal::Int(right_i) => if left_i < right_i { Literal::TRUE }
                                else { Literal::FALSE },
                            Literal::Float(right_f) => if (left_i as f32) < right_f { Literal::TRUE }
                            else { Literal::FALSE },
                            _ => panic!("expected numerical value")
                        },
                        Literal::Float(left_f) => match right {
                            // Literal::Char(_) => todo!(),
                            // Literal::String(_) => todo!(),
                            // Literal::Date(_, _, _) => todo!(),
                            Literal::Int(right_i) => if left_f < right_i as f32 { Literal::TRUE }
                            else { Literal::FALSE },
                            Literal::Float(right_f) => if left_f < right_f { Literal::TRUE }
                            else { Literal::FALSE },
                            _ => panic!("expected numerical value")
                        },
                        _ => panic!("expected numerical value")
                    }
                },
                TokenType::Greater => {
                    let left = left.interpret(env);
                    let right = right.interpret(env);
                    match left {
                        // Literal::Char(_) => todo!(),
                        // Literal::String(_) => todo!(),
                        // Literal::Date(_, _, _) => todo!(),
                        Literal::Int(left_i) => match right {
                            // Literal::Char(_) => todo!(),
                            // Literal::String(_) => todo!(),
                            // Literal::Date(_, _, _) => todo!(),
                            Literal::Int(right_i) => if left_i > right_i { Literal::TRUE }
                                else { Literal::FALSE },
                            Literal::Float(right_f) => if (left_i as f32) > right_f { Literal::TRUE }
                            else { Literal::FALSE },
                            _ => panic!("expected numerical value")
                        },
                        Literal::Float(left_f) => match right {
                            // Literal::Char(_) => todo!(),
                            // Literal::String(_) => todo!(),
                            // Literal::Date(_, _, _) => todo!(),
                            Literal::Int(right_i) => if left_f > right_i as f32 { Literal::TRUE }
                            else { Literal::FALSE },
                            Literal::Float(right_f) => if left_f > right_f { Literal::TRUE }
                            else { Literal::FALSE },
                            _ => panic!("expected numerical value")
                        },
                        _ => panic!("expected numerical value")
                    }
                },
                TokenType::LessEqual => {
                    let left = left.interpret(env);
                    let right = right.interpret(env);
                    match left {
                        // Literal::Char(_) => todo!(), just use normal equal
                        // Literal::String(_) => todo!(), plus Less operation
                        // Literal::Date(_, _, _) => todo!(),
                        Literal::Int(left_i) => match right {
                            // Literal::Char(_) => todo!(),
                            // Literal::String(_) => todo!(),
                            // Literal::Date(_, _, _) => todo!(),
                            Literal::Int(right_i) => if left_i <= right_i { Literal::TRUE }
                                else { Literal::FALSE },
                            Literal::Float(right_f) => if (left_i as f32) <= right_f { Literal::TRUE }
                            else { Literal::FALSE },
                            _ => panic!("expected numerical value")
                        },
                        Literal::Float(left_f) => match right {
                            // Literal::Char(_) => todo!(),
                            // Literal::String(_) => todo!(),
                            // Literal::Date(_, _, _) => todo!(),
                            Literal::Int(right_i) => if left_f <= right_i as f32 { Literal::TRUE }
                            else { Literal::FALSE },
                            Literal::Float(right_f) => if left_f <= right_f { Literal::TRUE }
                            else { Literal::FALSE },
                            _ => panic!("expected numerical value")
                        },
                        _ => panic!("expected numerical value")
                    }
                },
                TokenType::GreaterEqual => {
                    let left = left.interpret(env);
                    let right = right.interpret(env);
                    match left {
                        // Literal::Char(_) => todo!(),
                        // Literal::String(_) => todo!(),
                        // Literal::Date(_, _, _) => todo!(),
                        Literal::Int(left_i) => match right {
                            // Literal::Char(_) => todo!(),
                            // Literal::String(_) => todo!(),
                            // Literal::Date(_, _, _) => todo!(),
                            Literal::Int(right_i) => if left_i >= right_i { Literal::TRUE }
                                else { Literal::FALSE },
                            Literal::Float(right_f) => if (left_i as f32) >= right_f { Literal::TRUE }
                            else { Literal::FALSE },
                            _ => panic!("expected numerical value")
                        },
                        Literal::Float(left_f) => match right {
                            // Literal::Char(_) => todo!(),
                            // Literal::String(_) => todo!(),
                            // Literal::Date(_, _, _) => todo!(),
                            Literal::Int(right_i) => if left_f >= right_i as f32 { Literal::TRUE }
                            else { Literal::FALSE },
                            Literal::Float(right_f) => if left_f >= right_f { Literal::TRUE }
                            else { Literal::FALSE },
                            _ => panic!("expected numerical value")
                        },
                        _ => panic!("expected numerical value")
                    }
                },
                // TokenType::MOD => { TODO: what do these do anyways? remainder and integer division? are they even operators and not functions?
                //     let left = left.interpret(env);
                //     let right = right.interpret(env);
                //     match left {
                //         Literal::Int(left_i) => match right {
                //             Literal::Int(right_i) => Literal::Int(left_i - right_i),
                //             Literal::Float(right_f) => Literal::Float(left_i as f32 - right_f),
                //             _ => panic!("expected numerical value")
                //         },
                //         Literal::Float(left_f) => match right {
                //             Literal::Int(right_i) => Literal::Float(left_f - right_i as f32),
                //             Literal::Float(right_f) => Literal::Float(left_f - right_f),
                //             _ => panic!("expected numerical value")
                //         },
                //         _ => panic!("expected numerical value")
                //     }
                // },
                // TokenType::DIV => {
                //     let left = left.interpret(env);
                //     let right = right.interpret(env);
                //     match left {
                //         Literal::Int(left_i) => match right {
                //             Literal::Int(right_i) => Literal::Int(left_i - right_i),
                //             Literal::Float(right_f) => Literal::Float(left_i as f32 - right_f),
                //             _ => panic!("expected numerical value")
                //         },
                //         Literal::Float(left_f) => match right {
                //             Literal::Int(right_i) => Literal::Float(left_f - right_i as f32),
                //             Literal::Float(right_f) => Literal::Float(left_f - right_f),
                //             _ => panic!("expected numerical value")
                //         },
                //         _ => panic!("expected numerical value")
                //     }
                // },
                TokenType::AND => {
                    let right = right.interpret(env);
                    match left.interpret(env) {
                        Literal::TRUE => match right {
                            Literal::TRUE => Literal::TRUE,
                            Literal::FALSE => Literal::FALSE,
                            _ => panic!("expected boolean value")
                        },
                        Literal::FALSE => match right {
                            Literal::TRUE | Literal::FALSE => Literal::FALSE,
                            _ => panic!("expected boolean value")
                        },
                        _ => panic!("expected boolean value")
                    }
                },
                TokenType::OR => {
                    let right = right.interpret(env);
                    match left.interpret(env) {
                        Literal::TRUE => match right {
                            Literal::TRUE | Literal::FALSE => Literal::TRUE,
                            _ => panic!("expected boolean value")
                        },
                        Literal::FALSE => match right {
                            Literal::TRUE => Literal::TRUE,
                            Literal::FALSE => Literal::FALSE,
                            _ => panic!("expected boolean value")
                        },
                        _ => panic!("expected boolean value")
                    }
                },
                _ => todo!()
            },
            Expr::Grouping(inner) => inner.interpret(env),
            Expr::IdentExpr(name) => env.get(name.lexeme.clone()).expect("reference to undefined variable").clone(),
            Expr::Literal(lit) => lit.clone(),
        }
    }
}