use std::io::{self, Write};

mod expansion;

fn read_input() -> String {
    let mut input_string: String = String::new();
    std::io::stdin().read_line(&mut input_string).ok().expect("Could not read input");
    return input_string.trim().to_string();
}

fn lex(input: String) -> String {
    return input
}

fn expand(input: String) -> String {
    expansion::brace_expand();
    return input
}

fn execute(input: String) -> i32{
    println!("{}", input);
    return 0
}

fn main() {
    loop {
        print!("$> ");
        io::stdout().flush().unwrap();
        let input_string: String = read_input();
        let retval: i32 = execute(expand(lex(input_string)));
        println!("{}", retval);
    }
}
