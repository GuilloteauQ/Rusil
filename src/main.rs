pub mod errors;
pub mod functions;
pub mod tokens;
pub mod types;
use crate::errors::*;
use crate::tokens::*;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

// for argument in env::args()
fn main() {
    assert_eq!(env::args().len(), 2);
    let args: Vec<_> = env::args().collect();
    let mut file = File::open(args[1].to_string()).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();

    let e = Expr::token_tree(code.as_str());
    // println!("Tree: {:?}", e);
    let result = e.exec();
    match result {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e.description()),
    }
}
