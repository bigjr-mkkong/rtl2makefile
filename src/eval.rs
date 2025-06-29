use lisp_parser::LispObject;
use crate::types::*;

fn eval_let(curr: &Vec<LispObject>, env: &mut sym_tab) -> symbol {
    //handle "reg" and "wire"
    todo!()
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
