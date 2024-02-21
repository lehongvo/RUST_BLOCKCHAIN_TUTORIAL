#[derive(Debug)]
enum status {
    Low,
    Medium,
    Hight
}
#[derive(Debug)]
pub struct Evaluate {
    name: String,
    age: u8,
    gmail: String,
    meal: super::Breakfast
}

impl Evaluate {
    pub fn store_evaluate(name: String, age: u8, gmail: String, meal: super::Breakfast) -> Evaluate {
        let evaluate: Evaluate = Evaluate {
            name: name,
            age: age,
            gmail: gmail,
            meal: meal,
        };
        return evaluate;
    }
}