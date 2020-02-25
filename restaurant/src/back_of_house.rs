use crate::front_of_house::serving;

pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    // Since fruit is private, we need this function to construct a Breakfast
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

// For public enums, all variants are public automatically
pub enum Appetizer {
    Soup,
    Salad,
}

fn fix_incorrect_order() {
    cook_order();
    serving::serve_order();
}

fn cook_order() {}
