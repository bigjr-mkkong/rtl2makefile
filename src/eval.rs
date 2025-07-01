use lisp_parser::LispObject;
use crate::types::*;
use crate::frontend::*;
use crate::generator::circgen;
use std::io::{self, Write};
use crate::generator::{Final_out, finalize};

fn eval_let_wire(deflist: &Vec<LispObject>, env: &mut sym_tab) {

    for var in &deflist[1..]{
        match var {
            LispObject::List(_) => {
                println!("Variable name cannot be list");
                panic!();
            }

            LispObject::String(var_name) => {
                if let Some(_) = env.get(&var_name) {
                    println!("Variable {} already defined", var_name);
                    panic!();
                } else {
                    env.insert(&var_name, rtl_types::wire);
                }
            }
        }
    }
}

fn eval_let_reg(deflist: &Vec<LispObject>, env: &mut sym_tab) {

    for var in &deflist[1..]{
        match var {
            LispObject::List(_) => {
                println!("Variable name cannot be list");
                panic!();
            }

            LispObject::String(var_name) => {
                if let Some(_) = env.get(&var_name) {
                    println!("Variable {} already defined", var_name);
                    panic!();
                } else {
                    env.insert(&var_name, rtl_types::dff);
                }
            }
        }
    }
}
fn eval_let(curr: &Vec<LispObject>, env: &mut sym_tab) -> symbol {
    //handle "reg" and "wire"
    let curr_list = &curr[1];

    match curr_list {
        LispObject::String(_) => {
            println!("Definition list cannot be a single string");
            panic!();
        }

        LispObject::List(deflist) => {
            for defunit in deflist {
                if let LispObject::String(_) = defunit {
                    println!("Definition list cannot be a single string");
                    panic!();
                }

                if let LispObject::List(li_defunit) = defunit{
                    let deftyp = &li_defunit[0];
                    if let LispObject::List(_) = deftyp {
                        println!("Definition type cannot be a list");
                        panic!();
                    }

                    if let LispObject::String(deftyp_str) = deftyp{
                        match deftyp_str.as_str() {
                            "wire" => {
                                eval_let_wire(li_defunit, env);
                            },

                            "reg" => {
                                eval_let_reg(li_defunit, env);
                            },
                            
                            _ => {
                                println!("Unsupported type {}", deftyp_str);
                                panic!();
                            }
                        }
                    }
                }
            }
        }
    }

    symbol{
        literal: None,
        typ:rtl_types::nodef,
        val: None
    }
}

fn apply<W: Write>(op: &str, list: &Vec<LispObject>, env: &mut sym_tab, buf: &mut Final_out<W>) -> symbol {
    //handle circuit connection
    //for now each oprand only correspond to 2 parameters
    let mut symvec: Vec<symbol> = Vec::new();

    for parms in &list[1..]{
        symvec.push(eval::<W>(parms, env, buf));
    }

    match op {
        "circ" => {
            finalize(env, buf);
            symbol{
                literal: None,
                typ: rtl_types::nodef,
                val: None
            }
        },

        "conn"| 
        "+"|"-"|"*"|"/"|
        "&"|"|"|"!" => {
            circgen(op, &symvec, env, buf)
        }
        _ => {
            println!("Unsupported oprand {}", op);
            panic!();
        }
    }
}


pub fn eval<W: Write>(curr: &LispObject, env: &mut sym_tab, buf: &mut Final_out<W>) -> symbol{
    match curr {
        LispObject::String(sstr) => {
            if let Ok(n) = sstr.parse::<i32>() {
                symbol{
                    literal: None,
                    typ: rtl_types::constant,
                    val: Some(114514)
                }
            } else {
                if let Some(ssym) = env.get(sstr) {
                    ssym.clone()
                } else {
                    println!("Symbol {} undefined", sstr);
                    panic!();
                }
            }
        }

        LispObject::List(list) => {
            match &list[0] {
                LispObject::String(op) => {
                    match op.as_str(){
                        "let" => {
                            eval_let(list, env);
                            eval(&list[2], env, buf)
                        }

                        "circ"|"conn"| 
                        "+"|"-"|"*"|"/"|
                        "&"|"|"|"!" => {
                            apply::<W>(op, list, env, buf)
                        }

                        _ => {
                            println!("Unsupported oprand {}", op);
                            panic!();
                        }
                    }
                }

                LispObject::List(_) => {
                    println!("Failed to evaluate program");
                    panic!();
                }
            }
        }
    }
}
