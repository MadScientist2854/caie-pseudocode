use super::expr::Expr;

pub trait PPrint {
    fn prettify(&self) -> String;
}

impl PPrint for Expr {
    fn prettify(&self) -> String {
        match self {
            Expr::Unary(operator, right) => format!("( {} {} )", operator.lexeme, (**right).prettify()),
            Expr::Binary(left, operator, right) => format!("( {} {} {} )", operator.lexeme, (**left).prettify(), (**right).prettify()),
            Expr::Grouping(inner) => format!("( {} )", (**inner).prettify()),
            Expr::IdentExpr(name) => name.lexeme.clone(),
            Expr::FnCall(_, _) => todo!(),
            Expr::Literal(value) => value.clone().to_string(),
            Expr::ArrIdx(_, _, _) => todo!(),
            Expr::ArrType(_, _, _) => todo!(),
        }
    }
}