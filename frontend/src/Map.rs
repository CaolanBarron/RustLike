use fundamentals::position as pos;


struct Map {
    base: String,
    entities: Vec<(pos::Position, char)>,
    
}

impl Map {
    
    pub fn display_base(&self) {
       print!("{}", self.base) 
    }

    pub fn refresh() {
        todo!()
    }
}

#[cfg(test)]
mod display_tests {
    use super::Map;


    fn test_map() -> Map{
        Map {
            base: "----s---------d--------d---f------".to_string(),
            entities: vec![],
        }
    }

    #[test]
    fn display_base_test() {
        let tm = test_map();

        tm.display_base();
    }
}