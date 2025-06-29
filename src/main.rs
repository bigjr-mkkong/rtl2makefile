use std::io;
use frontend::*;


fn main() {
    let mut prog: Program = Program::new();

    let _ = prog.read_n_parse();
    prog.print_ast();
}

mod frontend;
mod types;
mod eval;
