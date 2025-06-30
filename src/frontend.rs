use std::io;
use lisp_parser::*;
use crate::types::*;

pub struct Program{
    ast: Vec<LispObject>,
}

impl Program{
    pub fn new() -> Self{
        Self{
            ast: Vec::new()
        }
    }

    pub fn get_ast(&self) -> &Vec<LispObject> {
        &self.ast
    }


    fn read_eof(prog: &mut String) -> io::Result<()> {
        let mut instr: String = String::new();
        let stdin = io::stdin();
        loop{
            let rl_ret = stdin.read_line(&mut instr);
            match rl_ret {
                Err(ecode) => {
                    println!("Failed to read from input: {}", ecode);
                    return Err(ecode);
                },
                Ok(byte_cnt) => {
                    if byte_cnt == 0 {
                        break;
                    } else {
                        let cleaned = instr.replace("\n", " ");
                        prog.push_str(&cleaned);
                        instr.clear();
                    }
                }

            }
        }

        Ok(())
    }

    pub fn read_n_parse(&mut self) {
        let mut program: String = String::new();
        Self::read_eof(&mut program).expect("Reader failed");

        let program = program.as_str();

        let pret = lisp_parser::parse_lisp_program(program);
        match pret {
            Ok(ast) => {
                self.ast = ast;
            },
            Err(_) => {
                panic!("Parse failed");
            }
        }
    }

    fn ast_printer(u: &lisp_parser::LispObject) {
        match(u) {
            LispObject::String(s) => {
                println!("{}", s);
                return;
            },
            LispObject::List(v) => {
                for iter in v {
                    Self::ast_printer(iter);
                }
            }
        }

        return;
    }
    
    pub fn print_ast(&self) {
        for iter in &self.ast {
            Self::ast_printer(&iter);
        }
    }

}

