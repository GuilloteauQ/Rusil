use std::ops::{Add, Div, Mul, Not, Sub};

#[derive(Debug, PartialEq)]
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
    pub fn evaluate(&self) -> Self {
        match self {
            Expr::Add(x, y) => x.evaluate() + y.evaluate(),
            Expr::Sub(x, y) => x.evaluate() - y.evaluate(),
            Expr::Mul(x, y) => x.evaluate() * y.evaluate(),
            Expr::Div(x, y) => x.evaluate() / y.evaluate(),
            Expr::Equal(x, y) => Expr::Bool(x.evaluate() == y.evaluate()),
            Expr::Not(x) => !x.evaluate(),
            Expr::Number(x) => Expr::Number(*x),
            Expr::Bool(x) => Expr::Bool(*x),
            Expr::Str(x) => Expr::Str(x.to_string()),
            Expr::If(b, x, y) => {
                if let Expr::Bool(bb) = b.evaluate() {
                    if bb {
                        x.evaluate()
                    } else {
                        y.evaluate()
                    }
                } else {
                    panic!("'If': bad evaluation")
                }
            }
            _ => unimplemented!(),
        }
    }

    /// Returns the token tree associated with the string
    pub fn token_tree(s: &str) -> Self {
        dbg!(s);
        if s.chars().next() != Some('(') {
            // If it is a number
            println!("s: {}", s);
            let x = s.trim().parse::<i32>();
            if x.is_ok() {
                Expr::Number(x.unwrap())
            } else {
                Expr::Str(s.to_string())
            }
        } else {
            // If it is an expression
            // let str_expressions: Vec<&str> = s[1..s.len() - 1].split(' ').collect();
            let str_expressions: Vec<String> = get_expressions(&s[1..s.len() - 1].trim());
            // let str_expressions: Vec<String> =
            //     get_expressions(s.get(1..s.len() - 1).unwrap().to_string()); //[1..s.len() - 1]);
            println!("str_expressions: {:?}", str_expressions);
            let mut command = str_expressions[0].as_str();
            if str_expressions[0].chars().next() == Some('(') {
                // TODO
                let e = Expr::token_tree(&get_expressions(&str_expressions[0])[0]);
                let x = e.evaluate();
                println!("x: {:?}", x);
                match x {
                    Expr::Str(c) => match c.as_str() {
                        "+" => command = "+",
                        "-" => command = "-",
                        "*" => command = "*",
                        "/" => command = "/",
                        _ => panic!("oopsie"),
                    },
                    _ => panic!("so, this was possible ..."),
                }
                // command = x;
                // unimplemented!()
            }
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

                _ => unimplemented!(),
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
