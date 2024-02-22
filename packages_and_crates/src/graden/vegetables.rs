#[derive(Debug)]
pub struct Asparagus {
    tree: String,
    flower: String,
}

impl Asparagus {
    pub fn create_tree() -> Asparagus {
        let new_tree = Asparagus {
            tree: String::from("Rose"),
            flower: String::from("Rose_01"),
        };
        return new_tree;
    }

    pub fn get_tree(&self) {
        println!("tree is {}, flower is {}", self.tree, self.flower);
    }
}
