use std::io::{self, Write};
use frontend::*;
use eval::eval;
use types::{sym_tab, symbol};
use lisp_parser::LispProgramParsingResult;
use generator::Final_out;

fn main() {
    let mut prog: Program = Program::new();
    let mut symtab: sym_tab = sym_tab::new();
    let mut out = io::stdout().lock();
    let mut comp_result = Final_out{
        outbuf: out,
        watchlist: Vec::new()
    };

    prog.read_n_parse();

    for lprog in prog.get_ast() {
        eval(lprog, &mut symtab, &mut comp_result);
    }

}

mod frontend;
mod types;
mod eval;
mod generator;
