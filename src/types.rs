use std::collections::HashMap;
#[derive(Eq, PartialEq)]
#[derive(Clone, Copy)]
pub enum rtl_types{
    wire,
    dff,
    constant,
    nodef
}

#[derive(Clone)]
pub struct symbol{
    pub typ: rtl_types,
    pub val: Option<i32>,
    pub literal: Option<String>,
}

pub struct sym_tab{
    tab: HashMap<String, symbol>
}

impl sym_tab{
    pub fn new() -> Self {
        Self{
            tab: HashMap::new()
        }
    }

    pub fn insert(&mut self, name: &String, var_typ: rtl_types) {
        let sname = name.to_string();
        let sym = match var_typ {
            rtl_types::wire => {
                symbol {
                    literal : Some(sname.to_owned()),
                    typ : rtl_types::wire,
                    val : None
                }
            }

            rtl_types::dff => {
                symbol {
                    literal : Some(sname.to_owned()),
                    typ: rtl_types::dff,
                    val: None
                }
            }

            rtl_types::constant => {
                symbol {
                    literal: None,
                    typ: rtl_types::constant,
                    val: Some(name.parse::<i32>().expect("Symbol is not a constant"))
                }
            }

            rtl_types::nodef => {
                symbol {
                    literal: None,
                    typ: rtl_types::nodef,
                    val: None
                }
            }
        };
        self.tab.insert(sname, sym);
    }

    pub fn remove(&mut self, name: &String) {
        self.tab.remove(name);
    }

    pub fn get(&self, name: &str) -> Option<&symbol> {
        self.tab.get(name)
    }

    pub fn get_all_regs(&self) -> Vec<&symbol> {
        let mut retvec: Vec<&symbol> = Vec::new();
        for (k, v) in &self.tab {
            if let rtl_types::dff = v.typ {
                retvec.push(&v);
            }
        }

        retvec
    }
}
