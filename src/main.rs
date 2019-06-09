pub mod tokens;
use crate::tokens::*;

fn main() {
    // let s = "(/ (* 3 2) (+ 1 2))";
    let s = "((if (= 1 2) + -) 2 1)";
    let e = Expr::token_tree(s);
    println!("Token tree: {:?}", e);
    let result = e.evaluate();
    println!("{:?}", result);
}
