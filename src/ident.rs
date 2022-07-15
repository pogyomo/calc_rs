use std::collections::HashMap;

pub struct Ident {
    map: HashMap<String, u16>,
}

impl Ident {
    pub fn new() -> Ident {
        Ident { map: HashMap::new() }
    }

    pub fn register(&mut self, name: String, value: u16) {
        self.map.insert(name, value);
    }

    pub fn value(&self, name: &String) -> Option<u16>{
        match self.map.get(name) {
            Some(&value) => Some(value),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Ident;

    #[test]
    fn test_ident() {
        let key = "AnKey".to_string();
        let mut map = Ident::new();
        map.register(key.clone(), 0);

        assert!(map.value(&key) == Some(0));
        assert!(map.value(&"InvalidKey".to_string()) == None);
    }
}
