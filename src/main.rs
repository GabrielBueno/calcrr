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
                match parser::parse(&lexer::lex(&code)) {
                    Ok(stmt) => println!("Running"),
                    Err(err) => println!("{}", err)
                };
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
