use crate::types::*;
use rand::*;
use std::io::{self, Write};
pub struct Final_out<W: Write>{
    pub outbuf: W,
    pub watchlist: Vec<String>,
}

fn wname_mangling(env: &sym_tab) -> String {
    let mut rng = rand::rng();

    let mut new_name = format!("w{}", rng.random::<u32>());
    while let Some(_) = env.get(&new_name) {
        new_name = format!("w{}", rng.random::<u32>());
    }

    return new_name;
}

fn format_template(template: &str, args: &[&str]) -> String {
    let mut result = String::new();
    let mut parts = template.split("{}");
    let mut iter = args.iter();

    if let Some(first) = parts.next() {
        result.push_str(first);
    }

    for part in parts {
        if let Some(arg) = iter.next() {
            result.push_str(arg);
        } else {
            result.push_str("{}");
        }
        result.push_str(part);
    }

    result
}


fn gen_w2w_2p<W: Write>(op: &str, parm1: &symbol, parm2: &symbol,env: &sym_tab, buf: &mut Final_out<W>) -> symbol{
    let template = 
    "\n{}: {} {}
        @{}_val = $$(head -n 1 {}); \\
        {}_val = $$(head -n 1 {}); \\
        result = $$(({}_val {} {}_val)); \\
        echo $$result > {}
        \n";
    let osym = symbol{
        literal : Some(wname_mangling(env)),
        typ : rtl_types::wire,
        val : None
    };

    let p1str = match parm1.typ {
        rtl_types::wire => parm1.literal.clone().unwrap(),
        rtl_types::dff => parm1.literal.clone().unwrap() + &"_q".to_string(),
        _ => {
            println!("Failed to generate for {}", parm1.literal.as_ref().unwrap());
            panic!();
        }
    };
    let p2str = match parm2.typ {
        rtl_types::wire => parm2.literal.clone().unwrap(),
        rtl_types::dff => parm2.literal.clone().unwrap() + &"_q".to_string(),
        _ => {
            println!("Failed to generate for {}", parm2.literal.as_ref().unwrap());
            panic!();
        }
    };
    let ostr = match osym.typ {
        rtl_types::wire => osym.literal.clone().unwrap(),
        rtl_types::dff => {
            println!("Output to dff is not possible in wire assignment :(");
            panic!();
        },
        _ => {
            println!("Failed to generate for {}", osym.literal.as_ref().unwrap());
            panic!();
        }
    };

    let out = format_template(&template, 
        &[&ostr, &p1str, &p2str,
        &p1str, &p1str,
        &p2str, &p2str,
        &p1str, op, &p2str,
        &ostr]);

    let _ = write!(buf.outbuf, "{}", out);

    osym
}

fn gen_w2w_1p<W: Write>(op: &str, parm1: &symbol, env: &sym_tab, buf: &mut Final_out<W>) -> symbol{
    let template = 
    "\n{}: {}
        @{}_val = $$(head -n 1 {}); \\
        result = $$((~{}_val)); \\
        echo $$result > {}
        \n";
    let osym = symbol{
        literal : Some(wname_mangling(env)),
        typ : rtl_types::wire,
        val : None
    };
    let p1str = match parm1.typ {
        rtl_types::wire => parm1.literal.clone().unwrap(),
        rtl_types::dff => parm1.literal.clone().unwrap() + &"_q".to_string(),
        _ => {
            println!("Failed to generate for {}", parm1.literal.as_ref().unwrap());
            panic!();
        }
    };
    let ostr = match osym.typ {
        rtl_types::wire => osym.literal.clone().unwrap(),
        rtl_types::dff => {
            println!("Output to dff is not possible in wire assignment :(");
            panic!();
        },
        _ => {
            println!("Failed to generate for {}", osym.literal.as_ref().unwrap());
            panic!();
        }
    };

    let out = format_template(&template, 
        &[&ostr, &p1str,
        &p1str, &p1str,
        &p1str,
        &ostr]);

    let _ = write!(buf.outbuf, "{}", out);

    osym
}

fn gen_conn_w2d<W: Write>(parm1: &symbol, parm2: &symbol,env: &sym_tab, buf: &mut Final_out<W>) -> symbol{
    let template = 
    "\n{}: {}
        @{}_val = $$(head -n 1 {}); \\
        echo $${}_val > {} \\
         od -An -N4 -tu4 /dev/urandom | tr -d ' ' > {} \\
        \n";
    let osym = symbol{
        literal : None,
        typ : rtl_types::nodef,
        val : None
    };

    let p1str = match parm1.typ {
        rtl_types::dff => parm1.literal.clone().unwrap() + &"_d".to_string(),
        _ => {
            println!("Failed to generate for {}", parm1.literal.as_ref().unwrap());
            panic!();
        }
    };

    let p1_tagname = parm1.literal.clone().unwrap() + &"_tag".to_string();

    let p2str = match parm2.typ {
        rtl_types::wire => parm2.literal.clone().unwrap(),
        _ => {
            println!("Failed to generate for {}", parm2.literal.as_ref().unwrap());
            panic!();
        }
    };

    let out = format_template(&template, 
        &[&p1str, &p2str,
        &p1str, &p2str,
        &p1str, &p1str,
        &p1_tagname]);

    let _ = write!(buf.outbuf, "{}", out);

    osym
}

fn gen_conn_w2w<W: Write>(parm1: &symbol, parm2: &symbol, env: &sym_tab, buf: &mut Final_out<W>) -> symbol{
    let template = 
    "\n{}: {}
        @{}_val = $$(head -n 1 {}); \\
        result = $$(({}_val)); \\
        echo $$result > {}
        \n";
    let osym = symbol{
        literal : parm1.literal.clone(),
        typ : rtl_types::wire,
        val : None
    };
    let p2str = match parm2.typ {
        rtl_types::wire => parm2.literal.clone().unwrap(),
        _ => {
            println!("Failed to generate for {}", parm2.literal.as_ref().unwrap());
            panic!();
        }
    };

    let ostr = match parm1.typ {
        rtl_types::wire => parm1.literal.clone().unwrap(),
        rtl_types::dff => {
            println!("Output to dff is not possible in wire assignment :(");
            panic!();
        },
        _ => {
            println!("Failed to generate for {}", osym.literal.as_ref().unwrap());
            panic!();
        }
    };

    let out = format_template(&template, 
        &[&ostr, &p2str,
        &p2str, &p2str,
        &p2str,
        &ostr]);

    let _ = write!(buf.outbuf, "{}", out);

    osym
}

pub fn circgen<W: Write>(
    op: &str,
    invec: &Vec<symbol>,
    env: &sym_tab,
    buf: &mut Final_out<W>) -> symbol {
    match op {
        "+"|"-"|"*"|"/"|
        "&"|"|"=> {
            gen_w2w_2p(op, &invec[0], &invec[1], env, buf)
        }

        "!" => {
            gen_w2w_1p(op, &invec[0], env, buf)
        }

        "conn" => {
            if let rtl_types::wire = &invec[0].typ {
                gen_conn_w2w(&invec[0], &invec[1], env, buf)
            } else {
                gen_conn_w2d(&invec[0], &invec[1], env, buf)
            }
        }

        _ => {
            println!("Failed to generate circuit for op({})", op);
            panic!();
        }
    }
}

pub fn finalize<W: Write>(env: &sym_tab, buf: &mut Final_out<W>) {
    let template = 
    "\n{}: {}
        @{}_val = $$(head -n 1 {}); \\
        echo $${}_val > {}
        \n";
    let dff_vec = env.get_all_regs();
    for i in dff_vec {
        let reg_q = i.literal.clone().unwrap() + &"_q";
        let reg_d = i.literal.clone().unwrap() + &"_d";
        let reg_tag = i.literal.clone().unwrap() + &"_tag";

        let out = format_template(&template, 
            &[&reg_q, &reg_tag,
            &reg_q, &reg_d,
            &reg_q, &reg_q]);

        let _ = write!(buf.outbuf, "{}", out);
    }
}
