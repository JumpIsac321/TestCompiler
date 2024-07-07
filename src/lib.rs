use std::error::Error;
use std::fmt::Display;
use std::fs;

mod parser;
mod tokenizer;

#[derive(Debug, PartialEq)]
pub enum Token {
    IntLit(i32),
    Ident(String),
    Eof,
    Exit,
    Let,
    Semi,
    OP,
    CP,
}

pub enum Node {
    Program,
    Statement,
    Expesion,
    IntLit(i32),
    Exit,
    Let,
    Ident,
}

#[derive(Debug)]
pub enum CodeError {
    SyntaxError,
    FailureError,
}
impl Display for CodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeError::SyntaxError => write!(f, "Syntax Error"),
            CodeError::FailureError => write!(f, "Faliure Error"),
        }
    }
}
impl Error for CodeError {}

pub fn run(file_path: &String) -> Result<(), Box<dyn Error>> {
    let file_string = fs::read_to_string(file_path)?;
    println!("{}", file_string);
    let tokens = match tokenizer::tokenize(file_string) {
        Ok(t) => {
            println!("{:?}", t);
            t
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    if tokens[0] == Token::Exit
        && tokens[1] == Token::OP
        && tokens[3] == Token::CP
        && tokens[4] == Token::Semi
        && tokens[5] == Token::Eof
    {
        if let Token::IntLit(n) = tokens[2] {
            let code = format!(
                "\
.global _start
_start:
\tmov rdi,60
\tmov rdx,{}
\tsyscall
",
                n
            );
            fs::write("./a.asm", code)?;
        }
    }
    Ok(())
}
