mod front_of_house;

use crate::front_of_house::hosting;

mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("복숭아"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    use self::front_of_house::hosting as hosting_as;
    hosting_as::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("호밀빵");
    println!("{:?}", meal);
    meal.toast = String::from("밀빵");
    println!("{:?}", meal);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

pub use crate::front_of_house::hosting as PubHosting;
