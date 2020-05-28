use std::io;
use std::io::Write;

mod repl;
mod lexer;
mod parser;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        match repl::read() {
            repl::Input::Code(code) =>  {
                parser::parse(lexer::lex(&code));
            },
            repl::Input::Quit => {
                println!("Bye\n");
                break;
            },
            repl::Input::Error(err) => {
                println!("An error ocurred: {}\n Bye...\n", err);
                break;
            },
        }
    }
}
