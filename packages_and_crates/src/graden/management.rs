use super::vegetables::Asparagus;

pub struct Tree {
    count: u8,
    owner: String,
    asparagus: Asparagus,
}

impl Tree {
    pub fn store(asparagus: Asparagus) -> Tree {
        let tree = Tree {
            count: 10,
            owner: String::from("Vo Le Hong"),
            asparagus: asparagus,
        };
        return tree;
    }
    pub fn get_info(&self) {
        println!(
            "asparagus is {:?}, count is {}, owner is {}",
            self.asparagus, self.count, self.owner
        );
    }
}
