use rand::{Rng, RngCore, SeedableRng};

mod front_house;
mod hosting;

use crate::hosting::{Breakfast, Salad};
use crate::hosting::hosting:: {add_to_hosting, update_to_hosting};

fn eat_at_restaurant() {
    let mut order = Breakfast::monday("Fish");
    order.toast = String::from("Chicken");

    let order1 = Breakfast{
        toast: String::from("Chicken"),
        fruit: String::from("Apple")
    };

    let mut order2 = Salad::Salad;
    order2 = Salad::Soup;

    add_to_hosting();
    update_to_hosting();

}