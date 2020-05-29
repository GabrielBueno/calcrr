use std::io;
use std::io::Write;

mod repl;
mod lexer;
mod parser;
mod calc;

fn main() {
    let mut calc = calc::Calc::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        match repl::read() {
            repl::Input::Code(code) =>  {
                match lexer::lex(&code) {
                    Ok(tokens) => {
                        match parser::parse(&tokens) {
                            Ok(stmt) => {
                                match calc.run(&stmt) {
                                    Ok(val)  => println!("= {}\n", val),
                                    Err(err) => println!("{}\n", err)
                                }
                            },
                            Err(err) => println!("{}\n", err)
                        };
                    },
                    Err(err) => println!("{}\n", err)
                }
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
