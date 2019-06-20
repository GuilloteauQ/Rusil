use crate::errors::*; // type_errors::TypeError;
use crate::functions::*;
use crate::types::*;
use std;
use std::collections::HashMap;
use std::fmt;
use std::io::{prelude, Write};
use std::io::{stdin, stdout};
use std::ops::{Add, Div, Mul, Not, Sub};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Expr {
    Number(i32),
    Bool(bool),
    Var(String),
    Str(String),
    EnumElement(String),
    // ------------------------------
    Add(Box<Expr>, Box<Expr>, String),
    Sub(Box<Expr>, Box<Expr>, String),
    Mul(Box<Expr>, Box<Expr>, String),
    Div(Box<Expr>, Box<Expr>, String),
    Mod(Box<Expr>, Box<Expr>, String),
    Equal(Box<Expr>, Box<Expr>, String),
    GreaterThan(Box<Expr>, Box<Expr>, String),
    GreaterEqualThan(Box<Expr>, Box<Expr>, String),
    LessThan(Box<Expr>, Box<Expr>, String),
    LessEqualThan(Box<Expr>, Box<Expr>, String),
    NEqual(Box<Expr>, Box<Expr>, String),
    And(Box<Expr>, Box<Expr>, String),
    Or(Box<Expr>, Box<Expr>, String),
    Not(Box<Expr>, String),
    If(Box<Expr>, Box<Expr>, Box<Expr>, String),
    Let(Box<Expr>, Box<Expr>, String),
    Set(Box<Expr>, Box<Expr>, String),
    Sequence(Vec<Box<Expr>>, String),
    For(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>, String),
    While(Box<Expr>, Box<Expr>, String),
    Define(Box<Expr>, Vec<Box<Expr>>, Box<Expr>, String),
    Call(Box<Expr>, Vec<Box<Expr>>, String),
    Print(Vec<Box<Expr>>),
    Enum(Box<Expr>, Vec<Box<Expt>>, String),
    Input,
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
            Expr::Var(_) => Type::Var,
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
    fn get_bool(&self, expr_str: String) -> Result<bool, LangError> {
        if let Expr::Bool(x) = self {
            Ok(*x)
        } else {
            Err(LangError::new_type_error(
                Type::Bool,
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

    /// Returns the string encapsulated in the expression
    /// If it is not a string, returns a TypeError
    fn get_var(&self, expr_str: String) -> Result<String, LangError> {
        if let Expr::Var(x) = self {
            Ok(x.to_string())
        } else {
            Err(LangError::new_type_error(
                Type::Var,
                self.get_type(),
                expr_str,
            ))
        }
    }

    /// Takes 2 Numbers and returns the number op(x, y)
    /// If type error returns a TypeError
    fn arith_operation<T, FOP: Fn(T, T) -> Expr, FGET: Fn(Expr) -> Result<T, LangError>>(
        self,
        other: Expr,
        op: FOP,
        get_f: FGET,
    ) -> Result<Expr, LangError> {
        let x = get_f(self)?;
        let y = get_f(other)?;
        Ok(op(x, y))
    }

    /// Execute the program represented by the token tree
    pub fn exec(&self) -> Result<Self, LangError> {
        let mut variables: HashMap<String, Expr> = HashMap::new();
        let mut functions: HashMap<String, Function> = HashMap::new();
        self.evaluate(&mut variables, &mut functions)
    }

    fn evaluate(
        &self,
        mut variables: &mut HashMap<String, Expr>,
        mut functions: &mut HashMap<String, Function>,
    ) -> Result<Self, LangError> {
        match self {
            Expr::Add(x, y, s) => x.evaluate(variables, functions)?.arith_operation(
                y.evaluate(variables, functions)?,
                |u, v| Expr::Number(u + v),
                |x| x.get_num(s.to_string()),
            ),
            Expr::Sub(x, y, s) => x.evaluate(variables, functions)?.arith_operation(
                y.evaluate(variables, functions)?,
                |u, v| Expr::Number(u - v),
                |x| x.get_num(s.to_string()),
            ),
            Expr::Mul(x, y, s) => x.evaluate(variables, functions)?.arith_operation(
                y.evaluate(variables, functions)?,
                |u, v| Expr::Number(u * v),
                |x| x.get_num(s.to_string()),
            ),
            Expr::Div(x, y, s) => x.evaluate(variables, functions)?.arith_operation(
                y.evaluate(variables, functions)?,
                |u, v| Expr::Number(u / v),
                |x| x.get_num(s.to_string()),
            ),
            Expr::Mod(x, y, s) => x.evaluate(variables, functions)?.arith_operation(
                y.evaluate(variables, functions)?,
                |u, v| Expr::Number(u % v),
                |x| x.get_num(s.to_string()),
            ),
            Expr::Equal(x, y, s) => x.evaluate(variables, functions)?.arith_operation(
                y.evaluate(variables, functions)?,
                |u, v| Expr::Bool(u == v),
                |x| x.get_num(s.to_string()),
            ),
            Expr::NEqual(x, y, s) => x.evaluate(variables, functions)?.arith_operation(
                y.evaluate(variables, functions)?,
                |u, v| Expr::Bool(u != v),
                |x| x.get_num(s.to_string()),
            ),
            Expr::GreaterThan(x, y, s) => x.evaluate(variables, functions)?.arith_operation(
                y.evaluate(variables, functions)?,
                |u, v| Expr::Bool(u > v),
                |x| x.get_num(s.to_string()),
            ),
            Expr::LessThan(x, y, s) => x.evaluate(variables, functions)?.arith_operation(
                y.evaluate(variables, functions)?,
                |u, v| Expr::Bool(u < v),
                |x| x.get_num(s.to_string()),
            ),
            Expr::GreaterEqualThan(x, y, s) => x.evaluate(variables, functions)?.arith_operation(
                y.evaluate(variables, functions)?,
                |u, v| Expr::Bool(u >= v),
                |x| x.get_num(s.to_string()),
            ),
            Expr::LessEqualThan(x, y, s) => x.evaluate(variables, functions)?.arith_operation(
                y.evaluate(variables, functions)?,
                |u, v| Expr::Bool(u <= v),
                |x| x.get_num(s.to_string()),
            ),
            Expr::And(x, y, s) => x.evaluate(variables, functions)?.arith_operation(
                y.evaluate(variables, functions)?,
                |u, v| Expr::Bool(u && v),
                |x| x.get_bool(s.to_string()),
            ),
            Expr::Or(x, y, s) => x.evaluate(variables, functions)?.arith_operation(
                y.evaluate(variables, functions)?,
                |u, v| Expr::Bool(u || v),
                |x| x.get_bool(s.to_string()),
            ),

            Expr::Not(x, _s) => Ok((!x.evaluate(variables, functions)?)?),
            Expr::Number(x) => Ok(Expr::Number(*x)),
            Expr::Bool(x) => Ok(Expr::Bool(*x)),
            Expr::Str(x) => Ok(Expr::Str(x.to_string())),
            Expr::Var(x) => {
                let p = variables.get(x);
                if let Some(e) = p {
                    e.clone().evaluate(&mut variables, functions)
                } else {
                    Ok(Expr::Var(x.to_string()))
                }
            }
            Expr::Let(name, x, s) => {
                let result = x.evaluate(variables, functions)?;
                let var_name = name.get_var(s.to_string())?;
                variables.insert(var_name, result);
                Ok(Expr::Empty)
            }
            Expr::Set(name, x, s) => {
                let result = x.evaluate(variables, functions)?;
                let var_name = name.get_var(s.to_string())?;

                let opt_previous = variables.insert(var_name.clone(), result);
                if opt_previous.is_none() {
                    Err(LangError::new_undefined_variable_error(
                        var_name,
                        s.to_string(),
                    ))
                } else {
                    Ok(Expr::Empty)
                }
            }
            Expr::Empty => Ok(Expr::Empty),
            Expr::Sequence(v, s) => {
                let mut result = None;
                for e in v {
                    result = Some(e.evaluate(variables, functions)?);
                }
                if result.is_none() {
                    Ok(Expr::Empty)
                } else {
                    Ok(result.unwrap())
                }
            }
            Expr::If(b, x, y, s) => {
                let bool_evaluated = b.evaluate(variables, functions)?;
                if let Expr::Bool(bb) = bool_evaluated {
                    if bb {
                        x.evaluate(variables, functions)
                    } else {
                        y.evaluate(variables, functions)
                    }
                } else {
                    Err(LangError::new_type_error(
                        Type::Bool,
                        bool_evaluated.get_type(),
                        s.to_string(),
                    ))
                }
            }
            Expr::For(var, begin, end, core, s) => {
                let inf = begin.get_num(s.to_string())?;
                let sup = end.get_num(s.to_string())?;
                let var_name = var.get_var(s.to_string())?;

                let previous_value = variables.insert(var_name.clone(), Expr::Number(inf));
                for i in inf..sup {
                    variables.insert(var_name.clone(), Expr::Number(i));
                    core.evaluate(variables, functions)?;
                }

                let _ = match previous_value {
                    Some(p) => variables.insert(var_name, p),
                    None => variables.remove(&var_name),
                };
                Ok(Expr::Empty)
            }
            Expr::While(bool_exp, core, s) => {
                let mut bool_val = bool_exp
                    .evaluate(variables, functions)?
                    .get_bool(s.to_string())?;
                while bool_val {
                    core.evaluate(&mut variables, &mut functions)?;
                    bool_val = bool_exp
                        .evaluate(variables, functions)?
                        .get_bool(s.to_string())?;
                }
                Ok(Expr::Empty)
            }

            Expr::Define(name, args, core, s) => {
                let func_name = name.get_var(s.to_string())?;
                let new_function = Function::new(
                    func_name.clone(),
                    args.iter()
                        .map(|a| {
                            a.evaluate(variables, functions)
                                .unwrap()
                                .get_var(s.to_string())
                                .unwrap()
                        })
                        .collect(),
                    core.clone(),
                );
                functions.insert(func_name, new_function);
                Ok(Expr::Empty)
            }
            Expr::Call(name, args, s) => {
                let func_name = name.get_var(s.to_string())?;

                let evaluated_args: Vec<Expr> = args
                    .iter()
                    .map(|a| a.evaluate(&mut variables, &mut functions).unwrap())
                    .collect();

                // println!("Calling function: {}, with args: {:?}", func_name, args);
                let function = functions.get(&func_name);
                if function.is_none() {
                    return Err(LangError::new_undefined_variable_error(
                        func_name.to_string(),
                        s.to_string(),
                    ));
                }
                let mut function = function.unwrap();
                let func_args = function.get_args();

                // Store the previous values of function.args

                let previous_values: Vec<Option<Expr>> = func_args
                    .iter()
                    .zip(evaluated_args.iter())
                    .map(|(arg_name, arg_passed_value)| {
                        variables.insert(arg_name.to_string(), arg_passed_value.clone())
                    })
                    .collect();

                // Apply the function
                let result = function
                    .get_core()
                    .evaluate(&mut variables, &mut functions)?;

                // Restore the values
                previous_values
                    .iter()
                    .zip(func_args.iter())
                    .for_each(|(v, name)| {
                        if v.is_some() {
                            variables.insert(name.to_string(), v.clone().unwrap());
                        } else {
                            variables.remove(name);
                        }
                    });
                // Return the result of the function call
                Ok(result)
            }
            Expr::Print(x) => {
                for e in x.iter() {
                    print!("{}", e.evaluate(variables, functions)?);
                }
                std::io::stdout().flush().unwrap();
                Ok(Expr::Empty)
            }
            Expr::Input => {
                let mut b = String::new();
                let _ = std::io::stdin().read_line(&mut b).unwrap();
                let x = b.trim().parse::<i32>();
                if x.is_ok() {
                    Ok(Expr::Number(x.unwrap()))
                } else {
                    Ok(Expr::Str(b.to_string()))
                }
            }
            Expr::Enum(enum_name, names, s) => {
                let str_enum_name = enum_name.get_str(s.to_string())?;
                for e in names.iter() {
                    let x = format!("{}.{}", str_enum_name, e.get_str(s.to_string()));
                    // TODO: HashSet, HashMap ?
                    // TODO: Push
                }
                Ok(Expr::Empty)
            }
        }
    }

    /// Returns the token tree associated with the string
    pub fn token_tree(s: &str) -> Self {
        // dblet p = s.replace("\n", " ");
        let p = s.replace("\n", " ");
        let trimed_command_exp = p.trim();
        if trimed_command_exp.chars().next().is_none() {
            return Expr::Empty;
        }
        if trimed_command_exp.chars().next() != Some('(') {
            // If it is a number
            let x = p.trim().parse::<i32>();
            if x.is_ok() {
                Expr::Number(x.unwrap())
            } else if p.trim().chars().take(1).next().unwrap() == '"' {
                Expr::Str(p[1..p.len() - 1].to_string())
            } else {
                Expr::Var(s.to_string())
            }
        } else {
            // If it is an expression
            let str_expressions: Vec<String> =
                get_expressions(&trimed_command_exp[1..trimed_command_exp.len() - 1]);
            // println!("EXPRESSIONS: {:?}", str_expressions);
            let command = str_expressions[0].as_str();
            // if str_expressions[0].chars().next() == Some('(') {
            //     panic!("Syntax not accepted ...");
            // } else {
            match command {
                    "+" => Expr::Add(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    "-" => Expr::Sub(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    "*" => Expr::Mul(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    "/" => Expr::Div(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    "%" => Expr::Mod(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    "=" => Expr::Equal(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    "!=" => Expr::NEqual(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    ">" => Expr::GreaterThan(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    ">=" => Expr::GreaterEqualThan(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    "<" => Expr::LessThan(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    "<=" => Expr::LessEqualThan(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    "&&" => Expr::And(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    "||" => Expr::Or(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),

                    "!" => Expr::Not(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    "if" => Expr::If(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[3].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    "let" => Expr::Let(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    "set" => Expr::Set(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        s.to_string(),
                    ),
                    "def" => Expr::Define(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        str_expressions[2..str_expressions.len() - 1].iter().map(|e| Box::new(Expr::token_tree(e.as_str().trim()))).collect::<Vec<Box<Expr>>>(),
                        Box::new(Expr::token_tree(str_expressions[str_expressions.len() - 1].as_str().trim())),
                        s.to_string(),
                    ),
                    "call" => Expr::Call(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        str_expressions.iter().skip(2).map(|e| Box::new(Expr::token_tree(e.as_str().trim()))).collect::<Vec<Box<Expr>>>(),
                        s.to_string(),
                    ),
                    "enum" => Expr::Enum(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        str_expressions.iter().skip(2).map(|e| Box::new(Expr::token_tree(e.as_str().trim()))).collect::<Vec<Box<Expr>>>(),
                        s.to_string(),
                    ),

                    "print" => Expr::Print(
                        str_expressions.iter().skip(1).map(|e| Box::new(Expr::token_tree(e.as_str().trim()))).collect::<Vec<Box<Expr>>>(),
                    ),
                    "input" => Expr::Input,
                    "while" => Expr::While(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    "for" => Expr::For(
                        Box::new(Expr::token_tree(str_expressions[1].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[2].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[3].as_str().trim())),
                        Box::new(Expr::token_tree(str_expressions[4].as_str().trim())),
                        trimed_command_exp.to_string(),
                    ),
                    _ => Expr::Sequence(str_expressions.iter().map(|e| Box::new(Expr::token_tree(e.as_str().trim()))).collect::<Vec<Box<Expr>>>(), trimed_command_exp.to_string())
                    // _ => Expr::Empty,
                }
            // }
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
    let mut just_pushed = false;
    let mut in_str = false;

    while let Some(c) = chars.next() {
        if c == ' ' && !in_expr && !just_pushed && !in_str {
            v.push(current_expr.trim().to_string());
            current_expr = String::new();
            just_pushed = false;
        } else if c == '(' {
            par_count += 1;
            current_expr.push(c);
            in_expr = par_count >= 1;
        } else if c == ')' && in_expr {
            par_count -= 1;
            in_expr = par_count != 0;
            current_expr.push(c);
            if !in_expr {
                v.push(current_expr.trim().to_string());
                current_expr = String::new();
                just_pushed = true;
            }
        } else if c == '"' {
            in_str = !in_str;
            current_expr.push(c);
        } else {
            current_expr.push(c);
            just_pushed = false;
        }
    }
    if current_expr.len() > 0 {
        v.push(current_expr.trim().to_string());
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
            Expr::token_tree("((let x 2) x)").exec().unwrap(),
            Expr::Number(2)
        );
    }

    #[test]
    fn test_for() {
        assert_eq!(
            Expr::token_tree("(for i 1 10 (i))").exec().unwrap(),
            Expr::Empty
        );
    }
}
