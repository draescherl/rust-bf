// Based on this tutorial : https://www.toptal.com/scala/writing-an-interpreter


use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide a filename.");
        process::exit(0);
    }

    let filename = &args[1];
    let source_code = fs::read_to_string(filename).expect("Could not find requested file.");
    // let lexed = lex(source_code);
    let lexed = lex("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.".to_string());
    let parsed = parse(lexed);

    let mut tape: Vec<u8> = vec![0; 1024];
    let mut pointer: usize = tape.len() / 2; // Init in the middle of the Vec to allow the user to go to the left
    println!("------ Running your brainf*ck program ------");
    run(&parsed, &mut tape, &mut pointer);
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
fn lex(code: String) -> Vec<Tokens> {
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



// Turn a sequence of tokens into a sequence of instructions
fn parse(tokens: Vec<Tokens>) -> Vec<Instructions> {
    let mut instructions: Vec<Instructions> = Vec::new();

    for (index, token) in tokens.iter().enumerate() {
        let instruction = match token {
            Tokens::MoveRight => Some(Instructions::MoveRight),
            Tokens::MoveLeft => Some(Instructions::MoveLeft),
            Tokens::Increment => Some(Instructions::Increment),
            Tokens::Decrement => Some(Instructions::Decrement),
            Tokens::Out => Some(Instructions::Print),
            Tokens::In => Some(Instructions::Input),
            Tokens::LoopEnter => Some(Instructions::Loop(
                let remove_previous_tokens = tokens
            )),
            _ => None // TODO: implement loop
        };

        match instruction {
            Some(instruction) => instructions.push(instruction),
            None => ()
        }
    }

    instructions
}


// Interpret the instructions
fn run(instructions: &Vec<Instructions>, tape: &mut Vec<u8>, pointer: &mut usize) {
    for instruction in instructions {
        match instruction {
            Instructions::MoveRight => *pointer += 1,
            Instructions::MoveLeft => *pointer -= 1,
            Instructions::Increment => tape[*pointer] += 1,
            Instructions::Decrement => tape[*pointer] -= 1,
            Instructions::Print => print!("{}", tape[*pointer] as char),
            Instructions::Input => {}, // TODO: implement user input
            Instructions::Loop(subinstructions) => {
                while tape[*pointer] != 0 {
                    run(&subinstructions, tape, pointer)
                }
            }
        }
    }
}