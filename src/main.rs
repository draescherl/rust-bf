// Based on this tutorial : https://www.toptal.com/scala/writing-an-interpreter


use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() == 1 {
        println!("Please provide a filename.");
        process::exit(0);
    }

    let filename = &args[1];
    let source_code = fs::read_to_string(filename)
        .expect("Could not find requested file.");
    println!("{}", source_code);

    let lexed = lex(source_code);
    let parsed = parse(lexed);
    for item in lexed {
        println!("{:?}", item);
    }
}


#[derive(Debug)]
enum Tokens {
    MoveRight,     // >
    MoveLeft,      // <
    Increment,     // +
    Decrement,     // -
    Out,           // .
    In,            // ,
    LoopEnter,     // [
    LoopExit,      // ]
}


enum Instructions {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Print,
    Input,
    Loop(Vec<Instructions>)
}


// Turn a sequence of characters into a sequence of tokens
fn lex2(code: String) -> Vec<Tokens> {
    let mut tokens = Vec::new();

    for char in code.chars() {
        let token = match char {
            '>' => Some(Tokens::MoveRight),
            '<' => Some(Tokens::MoveLeft),
            '+' => Some(Tokens::Increment),
            '-' => Some(Tokens::Decrement),
            '.' => Some(Tokens::Out),
            ',' => Some(Tokens::In),
            '[' => Some(Tokens::LoopEnter),
            ']' => Some(Tokens::LoopExit),
            _ => None
        };

        match token {
            Some(token) => tokens.push(token),
            None => (),
        }

    }

    tokens
}



// Turn a sequence of tokens into an AST
fn parse(tokens: Vec<Tokens>) -> Vec<Instructions> {
    let mut ast: Vec<Instructions> = Vec::new();

    ast
}


// Interpret the AST
fn run() {

}