#[derive(Debug)]
pub enum Salad {
    Galic_Salad,
    Potato_Soup,
    Lemon_Soup,
}

#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,
    pub fruit: String,
    pub salad: Salad,
}

impl Breakfast {
    pub fn meal_king() -> Breakfast {
        let meal: Breakfast = Breakfast {
            toast: String::from("Chicken"),
            fruit: String::from("Banana"),
            salad: Salad::Potato_Soup,
        };
        return meal;
    }

    pub fn meal_queen() -> Breakfast {
        let meal: Breakfast = Breakfast {
            toast: String::from("Beefsteak"),
            fruit: String::from("Apply"),
            salad: Salad::Lemon_Soup,
        };
        return meal;
    }

    pub fn update_toast(&mut self, toast: String, fruit: String) {
        self.toast = toast;
        self.fruit = fruit;
    }
}
