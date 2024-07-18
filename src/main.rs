use std::io::{self, Write};

mod eval;
mod lisp;
mod read;

fn main() {
    // read/eval/print
    // todo: exception
    loop {
        print!("toplevel> ");
        io::stdout().flush().unwrap();

        // read
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("panic");
        println!(">> {}", input); // debug

        let parsed = read::read(&mut input);

        println!(">> {:#?}", parsed); // debug

        // eval
        let value = eval::eval(parsed);

        // print
        // todo print cons cell
    }
}
