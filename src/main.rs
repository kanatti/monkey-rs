use lexer::Lexer;
use std::io::{self, BufRead, Write};
use token::Token;

mod lexer;
mod token;

static PROMPT: &'static str = ">>";

// 1. Show prompt
// 2. Read a line
// 3. Parse and output tokens till EOF
// 4. loop
fn repl() {
    loop {
        prompt();

        let line = read_line();
        if line.trim() == "exit" {
            break;
        }

        let mut lexer = Lexer::new(line.as_str());

        let mut token = lexer.next_token();
        while token != Token::EOF {
            println!("{:?}", token);
            token = lexer.next_token();
        }
    }
}

fn prompt() {
    print!("{}", PROMPT);
    io::stdout().flush().unwrap();
}

fn read_line() -> String {
    let mut line = String::new();

    io::stdin()
        .lock()
        .read_line(&mut line)
        .expect("Error reading");

    line
}

fn main() {
    println!("monkey-rs REPL!");
    repl();
}
