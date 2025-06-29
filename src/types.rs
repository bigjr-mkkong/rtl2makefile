use std::collections::HashMap;
#[derive(Eq, PartialEq)]
#[derive(Clone, Copy)]
pub enum rtl_types{
    wire,
    dff,
    constant
}

#[derive(Clone)]
pub struct symbol{
    pub typ: rtl_types,
    pub val: Option<i32>,
    pub literal: Option<String>
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
        };
        self.tab.insert(sname, sym);
    }

    pub fn remove(&mut self, name: &String) {
        self.tab.remove(name);
    }

    pub fn get(&self, name: &String) -> &symbol {
        self.tab.get(name).expect(&format!("Symbol {} not defined yet", name))
    }
}
