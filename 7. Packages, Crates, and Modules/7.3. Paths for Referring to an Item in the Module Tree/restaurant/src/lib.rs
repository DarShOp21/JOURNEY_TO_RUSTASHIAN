mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            //logic of the fn
        }
    }
}

fn deliver_order() {}

mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer{
        Soup,
        Salad
    }

    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
}

pub fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like to have {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");
    // The above will lead to error bcoz the seasonal_fruit is private

    
    //ENUMS
    let order = back_of_house::Appetizer::Soup;
}

