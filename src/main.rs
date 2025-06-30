use std::io;
use frontend::*;
use eval::eval;
use types::{sym_tab, symbol};
use lisp_parser::LispProgramParsingResult;

fn main() {
    let mut prog: Program = Program::new();
    let mut symtab: sym_tab = sym_tab::new();

    prog.read_n_parse();
    prog.print_ast();

    // for lprog in prog.get_ast() {
    //     eval(lprog, &mut symtab);
    // }
}

mod frontend;
mod types;
mod eval;
