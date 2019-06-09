use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Not, Sub};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Number(i32),
    Equal(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Bool(bool),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Str(String),
    Let(Box<Expr>, Box<Expr>, Box<Expr>),
    For(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
    Empty,
}

impl Add for Expr {
    type Output = Expr;

    fn add(self, other: Expr) -> Expr {
        if let Expr::Number(x) = self {
            if let Expr::Number(y) = other {
                Expr::Number(x + y)
            } else {
                panic!("Could not add these elements")
            }
        } else {
            panic!("Could not add these elements")
        }
    }
}

impl Sub for Expr {
    type Output = Expr;
    fn sub(self, other: Expr) -> Expr {
        if let Expr::Number(x) = self {
            if let Expr::Number(y) = other {
                Expr::Number(x - y)
            } else {
                panic!("Could not sub these elements")
            }
        } else {
            panic!("Could not sub these elements")
        }
    }
}

impl Div for Expr {
    type Output = Expr;
    fn div(self, other: Expr) -> Expr {
        if let Expr::Number(x) = self {
            if let Expr::Number(y) = other {
                Expr::Number(x / y)
            } else {
                panic!("Could not div these elements")
            }
        } else {
            panic!("Could not div these elements")
        }
    }
}

impl Mul for Expr {
    type Output = Expr;
    fn mul(self, other: Expr) -> Expr {
        if let Expr::Number(x) = self {
            if let Expr::Number(y) = other {
                Expr::Number(x * y)
            } else {
                panic!("Could not mul these elements")
            }
        } else {
            panic!("Could not mul these elements")
        }
    }
}

impl Not for Expr {
    type Output = Expr;
    fn not(self) -> Expr {
        if let Expr::Bool(x) = self {
            Expr::Bool(!x)
        } else {
            panic!("Could not mul these elements")
        }
    }
}

impl Expr {
    fn get_num(&self) -> i32 {
        if let Expr::Number(x) = self {
            *x
        } else {
            panic!("Nah")
        }
    }
    fn get_str(&self) -> String {
        if let Expr::Str(x) = self {
            x.to_string()
        } else {
            panic!("Nah")
        }
    }

    pub fn exec(&self) -> Self {
        let mut variables: HashMap<String, Expr> = HashMap::new();
        self.evaluate(&mut variables)
    }

    fn evaluate(&self, mut variables: &mut HashMap<String, Expr>) -> Self {
        match self {
            Expr::Add(x, y) => x.evaluate(variables) + y.evaluate(variables),
            Expr::Sub(x, y) => x.evaluate(variables) - y.evaluate(variables),
            Expr::Mul(x, y) => x.evaluate(variables) * y.evaluate(variables),
            Expr::Div(x, y) => x.evaluate(variables) / y.evaluate(variables),
            Expr::Equal(x, y) => Expr::Bool(x.evaluate(variables) == y.evaluate(variables)),
            Expr::Not(x) => !x.evaluate(variables),
            Expr::Number(x) => Expr::Number(*x),
            Expr::Bool(x) => Expr::Bool(*x),
            Expr::Str(x) => {
                let p = variables.get(x);
                if let Some(e) = p {
                    e.clone().evaluate(&mut variables)
                } else {
                    Expr::Str(x.to_string())
                }
            }
            Expr::Let(name, x, next) => {
                let result = x.evaluate(variables);
                // let var = name.evaluate(variables);
                let var_name = name.get_str();
                variables.insert(var_name, result);
                next.evaluate(variables)
            }
            Expr::Empty => Expr::Empty,
            Expr::If(b, x, y) => {
                if let Expr::Bool(bb) = b.evaluate(variables) {
                    if bb {
                        x.evaluate(variables)
                    } else {
                        y.evaluate(variables)
                    }
                } else {
                    panic!("'If': bad evaluation")
                }
            }
            Expr::For(var, begin, end, core, next) => {
                let inf = begin.get_num();
                let sup = end.get_num();
                for i in inf..sup {
                    variables.insert(var.get_str(), Expr::Number(i));
                    core.evaluate(variables);
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
                        Box::new(Expr::token_tree(str_expressions[1].as_str())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str())),
                    ),
                    "-" => Expr::Sub(
                        Box::new(Expr::token_tree(str_expressions[1].as_str())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str())),
                    ),
                    "*" => Expr::Mul(
                        Box::new(Expr::token_tree(str_expressions[1].as_str())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str())),
                    ),
                    "/" => Expr::Div(
                        Box::new(Expr::token_tree(str_expressions[1].as_str())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str())),
                    ),
                    "=" => Expr::Equal(
                        Box::new(Expr::token_tree(str_expressions[1].as_str())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str())),
                    ),
                    "!" => Expr::Not(Box::new(Expr::token_tree(str_expressions[1].as_str()))),
                    "if" => Expr::If(
                        Box::new(Expr::token_tree(str_expressions[1].as_str())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str())),
                        Box::new(Expr::token_tree(str_expressions[3].as_str())),
                    ),
                    "let" => Expr::Let(
                        Box::new(Expr::token_tree(str_expressions[1].as_str())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str())),
                        Box::new(Expr::token_tree(str_expressions[3].as_str())),
                    ),
                    "for" => Expr::For(
                        Box::new(Expr::token_tree(str_expressions[1].as_str())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str())),
                        Box::new(Expr::token_tree(str_expressions[3].as_str())),
                        Box::new(Expr::token_tree(str_expressions[4].as_str())),
                        Box::new(Expr::token_tree(str_expressions[5].as_str())),
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

    let mut chars = s.chars();
    let mut in_expr = false;
    let mut par_count = 0;

    while let Some(c) = chars.next() {
        if c == ' ' && !in_expr {
            v.push(current_expr);
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
    v.push(current_expr);
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
        assert_eq!(Expr::token_tree("(+ 1 1)").evaluate(), Expr::Number(2));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Expr::token_tree("(- 1 1)").evaluate(), Expr::Number(0));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Expr::token_tree("(* 2 1)").evaluate(), Expr::Number(2));
    }

    #[test]
    fn test_div() {
        assert_eq!(Expr::token_tree("(/ 4 2)").evaluate(), Expr::Number(2));
    }

    #[test]
    fn test_equal_false() {
        assert_eq!(Expr::token_tree("(= 4 2)").evaluate(), Expr::Bool(false));
    }

    #[test]
    fn test_equal_true() {
        assert_eq!(Expr::token_tree("(= 4 4)").evaluate(), Expr::Bool(true));
    }

    #[test]
    fn test_not_false() {
        assert_eq!(
            Expr::token_tree("(! (= 4 4))").evaluate(),
            Expr::Bool(false)
        );
    }

    #[test]
    fn test_not_true() {
        assert_eq!(Expr::token_tree("(! (= 4 2))").evaluate(), Expr::Bool(true));
    }

    #[test]
    fn test_if_true() {
        assert_eq!(
            Expr::token_tree("(if (= 2 2) 2 4)").evaluate(),
            Expr::Number(2)
        );
    }

    #[test]
    fn test_if_false() {
        assert_eq!(
            Expr::token_tree("(if (= 1 2) 2 4)").evaluate(),
            Expr::Number(4)
        );
    }

}
