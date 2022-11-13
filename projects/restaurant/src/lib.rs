mod front_of_house;


mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}

mod customer {
    use crate::front_of_house::hosting;
    use crate::back_of_house;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    
        let mut meal = back_of_house::Breakfast::summer("Rye");
    
        meal.toast = String::from("Wheat");
        println!("My meal: {}", meal.toast);
    
        // meal.seasonal_fruit = String::from("Strawberries");
    
        let order_1 = back_of_house::Appetizer::Salad;
        let order_1 = back_of_house::Appetizer::Soup;
    }
}



fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches")
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}