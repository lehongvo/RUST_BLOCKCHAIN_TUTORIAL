mod back_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        pub fruit: String
    }

    pub enum Salad {
        Soup,
        Salad
    }

    impl Breakfast {
        pub fn order_food() -> Breakfast {
            let meal = Breakfast {
                toast: String::from("chicken"),
                fruit: String::from("Banana")
            };
            return meal;
        }
    }
}

mod front_house;

use crate::back_house::{Breakfast, Salad};

fn eat_at_restaurant() {
    let mut meal: Breakfast = Breakfast::order_food();
    meal.toast = String::from("rice");

    let mut salas: Salad = Salad::Salad;
}