pub enum Input {
    Code(String),
    Quit,
    Error(&'static str)
}

pub fn read()-> Input {
    let mut buffer = String::new();

    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => (),
        Err(_) => return Input::Error("Error while reading stdin")
    };

    match buffer.as_str().trim() {
        "quit" => Input::Quit,
        code   => Input::Code(code.to_string())
    }
}