use lisp_parser::LispObject;
use crate::types::*;
use crate::frontend::*;

fn eval_let_wire(deflist: &Vec<LispObject>, env: &mut sym_tab) {

    for var in &deflist[1..]{
        match var {
            LispObject::List(_) => {
                println!("Variable name cannot be list");
                panic!();
            }

            LispObject::String(var_name) => {
                env.insert(&var_name, rtl_types::wire);
                println!("eval_let_reg: insert {} as wire", var_name);
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
                env.insert(&var_name, rtl_types::dff);
                println!("eval_let_reg: insert {} as reg", var_name);
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

fn apply(op: &str, list: &Vec<LispObject>, env: &mut sym_tab) -> symbol {
    //handle circuit connection
    todo!()
}


pub fn eval(curr: &LispObject, env: &mut sym_tab) -> symbol{
    match curr {
        LispObject::String(sstr) => {
            if let Ok(n) = sstr.parse::<i32>() {
                symbol{
                    literal: None,
                    typ: rtl_types::constant,
                    val: Some(n)
                }
            } else {
                env.get(sstr).clone()
            }
        }

        LispObject::List(list) => {
            match &list[0] {
                LispObject::String(op) => {
                    match op.as_str(){
                        "let" => {
                            eval_let(list, env)
                        }

                        "circ"|"conn"| 
                        "+"|"-"|"*"|"/"|
                        "&"|"|"|"!" => {
                            apply(op, list, env)
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
