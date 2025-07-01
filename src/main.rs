use std::io::{self, Write};
use frontend::*;
use eval::eval;
use types::{sym_tab, symbol};
use lisp_parser::LispProgramParsingResult;
use generator::Final_out;
use std::collections::HashSet;
use std::fs::File;

fn main() -> io::Result<()>{
    let mut prog: Program = Program::new();
    let mut symtab: sym_tab = sym_tab::new();
    let mut out: Vec<u8> = Vec::new();
    let mut comp_result = Final_out{
        outbuf: out,
        watchlist: Vec::new(),
        created_files: HashSet::new(),
        dir_created: false
    };

    prog.read_n_parse();

    for lprog in prog.get_ast() {
        eval(lprog, &mut symtab, &mut comp_result);
    }

    let generated = String::from_utf8(comp_result.outbuf).expect("failed to convert vec<u8> to string");

    let mut watchlist = File::create("./fsig/flist")?;
    let mut watchlist_str = String::new();

    let mut final_makefile = String::new();

    final_makefile.push_str(".PHONY: ");

    for line in &comp_result.watchlist {
        final_makefile.push_str(&format!("{} ", line));
        watchlist_str.push_str(&format!("{}\n", line));
    }
    final_makefile.push('\n');

    final_makefile.push_str(&generated);

    let mut file = File::create("./fsig/Makefile")?;
    file.write_all(final_makefile.as_bytes())?;

    watchlist.write_all(watchlist_str.as_bytes())?;

    Ok(())

}

mod frontend;
mod types;
mod eval;
mod generator;
