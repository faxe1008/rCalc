extern crate regex;

mod parser;
use crate::parser::calculator::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 1usize
    {
        match evaluate(&args[0])
        {
            Ok(r) => println!("Result: {}", r),
            Err(e) => println!("Error evaluating the expression: {}", e)
        }
    }
    else
    {
        println!("Please provide an expression to evaluate as one string without spaces. Example: 2*5");
    }
}
