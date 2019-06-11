use crate::errors::*; // type_errors::TypeError;
use crate::types::*;
use std;
use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Div, Mul, Not, Sub};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Expr {
    Add(Box<Expr>, Box<Expr>, String),
    Sub(Box<Expr>, Box<Expr>, String),
    Mul(Box<Expr>, Box<Expr>, String),
    Div(Box<Expr>, Box<Expr>, String),
    Number(i32),
    Equal(Box<Expr>, Box<Expr>, String),
    Not(Box<Expr>, String),
    Bool(bool),
    If(Box<Expr>, Box<Expr>, Box<Expr>, String),
    Str(String),
    Let(Box<Expr>, Box<Expr>, Box<Expr>, String),
    For(
        Box<Expr>,
        Box<Expr>,
        Box<Expr>,
        Box<Expr>,
        Box<Expr>,
        String,
    ),
    Empty,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(x) => write!(f, "{}", x),
            Expr::Bool(x) => write!(f, "{}", x),
            Expr::Str(x) => write!(f, "{}", x),
            _ => write!(f, ""),
        }
    }
}

impl Not for Expr {
    type Output = Result<Expr, LangError>;

    fn not(self) -> Result<Expr, LangError> {
        if let Expr::Bool(x) = self {
            Ok(Expr::Bool(!x))
        } else {
            Err(LangError::new_type_error(
                Type::Bool,
                self.get_type(),
                String::from(""),
            ))
        }
    }
}

impl Expr {
    /// Returns the type of the expression
    fn get_type(&self) -> Type {
        match *self {
            Expr::Number(_) => Type::Number,
            Expr::Str(_) => Type::Str,
            Expr::Bool(_) => Type::Bool,
            _ => Type::Expression,
        }
    }
    /// Returns the number encapsulated in the expression
    /// If it is not a number, returns a TypeError
    fn get_num(&self, expr_str: String) -> Result<i32, LangError> {
        if let Expr::Number(x) = self {
            Ok(*x)
        } else {
            Err(LangError::new_type_error(
                Type::Number,
                self.get_type(),
                expr_str,
            ))
        }
    }
    /// Returns the string encapsulated in the expression
    /// If it is not a string, returns a TypeError
    fn get_str(&self, expr_str: String) -> Result<String, LangError> {
        if let Expr::Str(x) = self {
            Ok(x.to_string())
        } else {
            Err(LangError::new_type_error(
                Type::Str,
                self.get_type(),
                expr_str,
            ))
        }
    }

    /// Takes 2 Numbers and returns the number op(x, y)
    /// If type error returns a TypeError
    fn arith_operation<F: Fn(i32, i32) -> i32>(
        self,
        other: Expr,
        op: F,
        expr_str: String,
    ) -> Result<Expr, LangError> {
        if let Expr::Number(x) = self {
            if let Expr::Number(y) = other {
                Ok(Expr::Number(op(x, y)))
            } else {
                Err(LangError::new_type_error(
                    Type::Number,
                    other.get_type(),
                    expr_str,
                ))
            }
        } else {
            Err(LangError::new_type_error(
                Type::Number,
                self.get_type(),
                expr_str,
            ))
        }
    }
    /// Execute the program represented by the token tree
    pub fn exec(&self) -> Result<Self, LangError> {
        let mut variables: HashMap<String, Expr> = HashMap::new();
        self.evaluate(&mut variables)
    }

    fn evaluate(&self, mut variables: &mut HashMap<String, Expr>) -> Result<Self, LangError> {
        match self {
            Expr::Add(x, y, s) => x.evaluate(variables)?.arith_operation(
                y.evaluate(variables)?,
                |u, v| u + v,
                s.to_string(),
            ),
            Expr::Sub(x, y, s) => x.evaluate(variables)?.arith_operation(
                y.evaluate(variables)?,
                |u, v| u - v,
                s.to_string(),
            ),
            Expr::Mul(x, y, s) => x.evaluate(variables)?.arith_operation(
                y.evaluate(variables)?,
                |u, v| u * v,
                s.to_string(),
            ),
            Expr::Div(x, y, s) => x.evaluate(variables)?.arith_operation(
                y.evaluate(variables)?,
                |u, v| u / v,
                s.to_string(),
            ),
            Expr::Equal(x, y, s) => {
                let x_result = x.evaluate(variables)?;
                let y_result = y.evaluate(variables)?;
                if x_result.get_type() == y_result.get_type() {
                    Ok(Expr::Bool(x_result == y_result))
                } else {
                    Err(LangError::new_type_error(
                        x_result.get_type(),
                        y_result.get_type(),
                        s.to_string(),
                    ))
                }
            }
            Expr::Not(x, _s) => Ok((!x.evaluate(variables)?)?),
            Expr::Number(x) => Ok(Expr::Number(*x)),
            Expr::Bool(x) => Ok(Expr::Bool(*x)),
            Expr::Str(x) => {
                let p = variables.get(x);
                if let Some(e) = p {
                    e.clone().evaluate(&mut variables)
                } else {
                    Ok(Expr::Str(x.to_string()))
                }
            }
            Expr::Let(name, x, next, s) => {
                let result = x.evaluate(variables)?;
                let var_name = name.get_str(s.to_string())?;
                variables.insert(var_name, result);
                next.evaluate(variables)
            }
            Expr::Empty => Ok(Expr::Empty),
            Expr::If(b, x, y, s) => {
                let bool_evaluated = b.evaluate(variables)?;
                if let Expr::Bool(bb) = bool_evaluated {
                    if bb {
                        x.evaluate(variables)
                    } else {
                        y.evaluate(variables)
                    }
                } else {
                    Err(LangError::new_type_error(
                        Type::Bool,
                        bool_evaluated.get_type(),
                        s.to_string(),
                    ))
                }
            }
            Expr::For(var, begin, end, core, next, s) => {
                let inf = begin.get_num(s.to_string())?;
                let sup = end.get_num(s.to_string())?;
                for i in inf..sup {
                    variables.insert(var.get_str(s.to_string())?, Expr::Number(i));
                    core.evaluate(variables)?;
                }
                next.evaluate(variables)
            }
        }
    }

    /// Returns the token tree associated with the string
    pub fn token_tree(s: &str) -> Self {
        // dbg!(s);
        if s.chars().next().is_none() {
            return Expr::Empty;
        }
        if s.chars().next() != Some('(') {
            // If it is a number
            let x = s.trim().parse::<i32>();
            if x.is_ok() {
                Expr::Number(x.unwrap())
            } else {
                Expr::Str(s.to_string())
            }
        } else {
            // If it is an expression
            let str_expressions: Vec<String> = get_expressions(&s[1..s.len() - 1].trim());
            let command = str_expressions[0].as_str();
            if str_expressions[0].chars().next() == Some('(') {
                panic!("Syntax not accepted ...");
            } else {
                match command {
                    "+" => Expr::Add(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        s.to_string(),
                    ),
                    "-" => Expr::Sub(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        s.to_string(),
                    ),
                    "*" => Expr::Mul(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        s.to_string(),
                    ),
                    "/" => Expr::Div(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        s.to_string(),
                    ),
                    "=" => Expr::Equal(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        s.to_string(),
                    ),
                    "!" => Expr::Not(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        s.to_string(),
                    ),
                    "if" => Expr::If(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[3].as_str().trim())),
                        s.to_string(),
                    ),
                    "let" => Expr::Let(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[3].as_str().trim())),
                        s.to_string(),
                    ),
                    "for" => Expr::For(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[3].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[4].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[5].as_str().trim())),
                        s.to_string(),
                    ),

                    _ => Expr::Empty,
                }
            }
        }
    }
}

fn get_expressions(s: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();

    let mut current_expr = String::new();
    let mut p = s.replace("\n", " ");
    p = p.replace("\t", " ");
    p = p.replace("  ", " ");
    // println!("P: {}", p);
    let mut chars = p.chars();
    let mut in_expr = false;
    let mut par_count = 0;

    while let Some(c) = chars.next() {
        if c == ' ' && !in_expr {
            v.push(current_expr.trim().to_string());
            current_expr = String::new();
        } else if c == '(' {
            par_count += 1;
            in_expr = par_count >= 1;
            current_expr.push(c);
        } else if c == ')' && in_expr {
            par_count -= 1;
            in_expr = par_count != 0;
            current_expr.push(c);
        } else {
            current_expr.push(c);
        }
    }
    v.push(current_expr.trim().to_string());
    for _ in 0..5 {
        v.push(String::from(""));
    }
    v
}

#[cfg(test)]
mod tests_tokens {
    use crate::tokens::*;

    #[test]
    fn test_add() {
        assert_eq!(Expr::token_tree("(+ 1 1)").exec().unwrap(), Expr::Number(2));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Expr::token_tree("(- 1 1)").exec().unwrap(), Expr::Number(0));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Expr::token_tree("(* 2 1)").exec().unwrap(), Expr::Number(2));
    }

    #[test]
    fn test_div() {
        assert_eq!(Expr::token_tree("(/ 4 2)").exec().unwrap(), Expr::Number(2));
    }

    #[test]
    fn test_equal_false() {
        assert_eq!(
            Expr::token_tree("(= 4 2)").exec().unwrap(),
            Expr::Bool(false)
        );
    }

    #[test]
    fn test_equal_true() {
        assert_eq!(
            Expr::token_tree("(= 4 4)").exec().unwrap(),
            Expr::Bool(true)
        );
    }

    #[test]
    fn test_not_false() {
        assert_eq!(
            Expr::token_tree("(! (= 4 4))").exec().unwrap(),
            Expr::Bool(false)
        );
    }

    #[test]
    fn test_not_true() {
        assert_eq!(
            Expr::token_tree("(! (= 4 2))").exec().unwrap(),
            Expr::Bool(true)
        );
    }

    #[test]
    fn test_if_true() {
        assert_eq!(
            Expr::token_tree("(if (= 2 2) 2 4)").exec().unwrap(),
            Expr::Number(2)
        );
    }

    #[test]
    fn test_if_false() {
        assert_eq!(
            Expr::token_tree("(if (= 1 2) 2 4)").exec().unwrap(),
            Expr::Number(4)
        );
    }

    #[test]
    fn test_let() {
        assert_eq!(
            Expr::token_tree("(let x 2 x)").exec().unwrap(),
            Expr::Number(2)
        );
    }

    #[test]
    fn test_for() {
        assert_eq!(
            Expr::token_tree("(for i 1 10 (i) i)").exec().unwrap(),
            Expr::Number(9)
        );
    }

}
