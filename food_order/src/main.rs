pub mod restaurant;

use crate::restaurant::{
    hosting:: {add_to_waitlist, seat_at_table},
    service:: {
        payment::{take_order, serve_order, take_payment},
        review::{comment, point_of_interest},
        history::{user_store, buyer_store, count_food},
        discount::{event, amount}
    }
};

fn main() {
    println!("-----Start hosting-----");
    add_to_waitlist();
    take_order();
    serve_order();
    seat_at_table();
    println!("---------------");

    println!("-----Start discount-----");
    event();
    amount();
    println!("---------------");

    println!("-----Start payment-----");
    take_order();
    serve_order();
    take_payment();
    println!("---------------");

    println!("-----Start review-----");
    comment();
    point_of_interest();
    println!("---------------");

    println!("-----Start history-----");
    user_store();
    buyer_store();
    count_food();
    println!("---------------");    
    
    println!("-----Start payment-----");
    take_payment();
    println!("-------Finish--------");
    
}
