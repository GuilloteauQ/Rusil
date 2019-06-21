use crate::tokens::Expr;

#[derive(Debug, PartialEq)]
pub(crate) enum Type {
    Number,
    Bool,
    Str,
    Expression,
    Var,
    Vector,
}
