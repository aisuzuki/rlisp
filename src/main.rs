use std::io::{self, Write};

mod eval;
mod lisp;
mod read;

fn main() {

    // todo: call init (install built-in functions, initialise memory)

    // read/eval/print
    // todo: exception
    loop {
        print!("toplevel> ");
        io::stdout().flush().unwrap();

        // read
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("panic");
        println!(">> {}", input); // debug

        match read::read(&mut input) {
            Ok(t) => {
                println!(">> {:#?}", t); // debug
                                         // eval
                let value = eval::eval(t);
                //print
                // todo print cons cell
            }
            Err(e) => {
                println!("!! {:#?}", e)
            }
        }
    }
    // TODO handle fatal error (e.g, memory allocation/free while gc)
}
