use std::collections::HashMap;
#[derive(Eq, PartialEq)]
pub enum rtl_types{
    wire,
    dff
}

pub struct sym_tab{
    tab: HashMap<String, rtl_types>
}

impl sym_tab{
    pub fn new() -> Self {
        Self{
            tab: HashMap::new()
        }
    }

    pub fn insert(&mut self, name: &String, typ: rtl_types) {
        self.tab.insert(name.to_string(), typ);
    }

    pub fn remove(&mut self, name: &String) {
        self.tab.remove(name);
    }

    pub fn get_type(&self, name: &String) -> &rtl_types {
        self.tab.get(name).expect(&format!("Symbol {} not defined yet", name))
    }
}
