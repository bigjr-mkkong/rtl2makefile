use crate::types::*;
use rand::*;
use std::io::{self, Write};
use std::fs::File;
use std::collections::HashSet;
pub struct Final_out<W: Write>{
    pub outbuf: W,
    pub watchlist: Vec<String>,
    pub created_files: HashSet<String>,
    pub dir_created: bool
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

fn create_file<W: Write>(name: &String, out: &mut Final_out<W>) {
    let dirpath = "./fsig/";
    if out.dir_created == false {
        let _ = std::fs::create_dir(dirpath);
        out.dir_created = true;
    }
    let full_path = dirpath.to_string() + name;
    if out.created_files.contains(&full_path) == false {
        if let Err(_) = File::create(&full_path){
            println!("Failed to create file: {}", full_path);
            panic!();
        }
        out.created_files.insert(full_path);
    }
}

fn gen_w2w_2p<W: Write>(op: &str, parm1: &symbol, parm2: &symbol,env: &sym_tab, buf: &mut Final_out<W>) -> symbol{
    let template = 
    "\n{}: {} {}
\t@{}_val=$$(head -n 1 {}); \\
\t{}_val=$$(head -n 1 {}); \\
\tresult=$$(({}_val {} {}_val)); \\
\techo $$result > {}
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

    create_file(&ostr, buf);
    create_file(&p1str, buf);
    create_file(&p2str, buf);

    osym
}

fn gen_w2w_1p<W: Write>(op: &str, parm1: &symbol, env: &sym_tab, buf: &mut Final_out<W>) -> symbol{
    let template = 
    "\n{}: {}
\t@{}_val=$$(head -n 1 {}); \\
\tresult=$$((~{}_val)); \\
\techo $$result > {}
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

    create_file(&ostr, buf);
    create_file(&p1str, buf);

    osym
}

fn gen_conn_w2d<W: Write>(parm1: &symbol, parm2: &symbol,env: &sym_tab, buf: &mut Final_out<W>) -> symbol{
    let template = 
    "\n{}: {}
\t@{}_val=$$(head -n 1 {}); \\
\techo $${}_val > {}; \\
\tod -An -N4 -tu4 /dev/urandom | tr -d ' ' > {} \\
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

    create_file(&p1str, buf);
    create_file(&p2str, buf);
    create_file(&p1_tagname, buf);

    let _ = write!(buf.outbuf, "{}", out);

    osym
}

fn gen_conn_w2w<W: Write>(parm1: &symbol, parm2: &symbol, env: &sym_tab, buf: &mut Final_out<W>) -> symbol{
    let template = 
    "\n{}: {}
\t@{}_val=$$(head -n 1 {}); \\
\tresult=$$(({}_val)); \\
\techo $$result > {}
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

    create_file(&ostr, buf);
    create_file(&p2str, buf);

    osym
}


fn gen_conn_d2w<W: Write>(parm1: &symbol, parm2: &symbol, env: &sym_tab, buf: &mut Final_out<W>) -> symbol{
    let template = 
    "\n{}: {}
\t@{}_val=$$(head -n 1 {}); \\
\tresult=$$(({}_val)); \\
\techo $$result > {}
        \n";
    let osym = symbol{
        literal : parm1.literal.clone(),
        typ : rtl_types::wire,
        val : None
    };
    let p2str = match parm2.typ {
        rtl_types::dff => parm2.literal.clone().unwrap() + &"_q",
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

    create_file(&ostr, buf);
    create_file(&p2str, buf);

    osym
}


fn gen_conn_d2d<W: Write>(parm1: &symbol, parm2: &symbol, env: &sym_tab, buf: &mut Final_out<W>) -> symbol{
    let template = 
    "\n{}: {}
\t@{}_val=$$(head -n 1 {}); \\
\tresult=$$(({}_val)); \\
\techo $$result > {}
        \n";
    let osym = symbol{
        literal : None,
        typ : rtl_types::nodef,
        val : None
    };
    let p2str = match parm2.typ {
        rtl_types::dff => parm2.literal.clone().unwrap() + &"_q",
        _ => {
            println!("Failed to generate for {}", parm2.literal.as_ref().unwrap());
            panic!();
        }
    };

    let ostr = match parm1.typ {
        rtl_types::dff => parm1.literal.clone().unwrap() + &"_d",
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

    create_file(&ostr, buf);
    create_file(&p2str, buf);

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
            match (&invec[0].typ, &invec[1].typ) {
                (rtl_types::wire, rtl_types::wire) => {
                    gen_conn_w2w(&invec[0], &invec[1], env, buf)
                }
                (rtl_types::dff, rtl_types::wire) => {
                    gen_conn_w2d(&invec[0], &invec[1], env, buf)
                }
                (rtl_types::wire, rtl_types::dff) => {
                    gen_conn_d2w(&invec[0], &invec[1], env, buf)
                }
                (rtl_types::dff, rtl_types::dff) => {
                    println!("dff to dff connection is not supported");
                    panic!();
                    // gen_conn_d2d(&invec[0], &invec[1], env, buf)
                }
                _ => {
                    println!("Unsupported type been used in conn");
                    panic!();
                }
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
\t@{}_val=$$(head -n 1 {}); \\
\techo $${}_val > {}
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
