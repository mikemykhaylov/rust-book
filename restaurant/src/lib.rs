mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
// have to specify individual fields to be public
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
// same with methods
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
// all enum values are public with pub
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // super refers to parent module
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

// use front_of_house::hosting; // when bringing in fns, specify path up to parent mod
// use back_of_house::Breakfast; // when bringing in structs or enums, specify full path

// Unless both mods have a type with same name, like "Result"
// use std::fmt; // fmt::Result
// use std::io; // io::Result<()>
// use std::io::Result as IoResult; // or we can just rename the type

// use std::{io::{self, Write}, fmt}; // nested imports to reduce number of `use`
// use std::* // or we can import EVERYTHING. Dangerous!

pub use front_of_house::hosting; // re-exporting allows users of our code
                                 // to reference submodules as if they were defined in their code

// Before this change, external code would have to access the add_to_waitlist
// function by using the path restaurant::front_of_house::hosting::add_to_waitlist().
// Now that this pub use has re-exported the hosting module from the root module,
// external code can use the path restaurant::hosting::add_to_waitlist() instead.

fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

    // allowed by use
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast); // can't use seasonal_fruit because it's private

    let order1 = back_of_house::Appetizer::Salad;
}

mod customer {
    fn eat_at_restaurant() {
        // hosting::add_to_waitlist();
        // not allowed, because `use` only creates symlink to current scope
    }
}
