mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}

        // fn seat_at_table() {}
    }

    // mod serving {
    //     fn take_order() {}

    //     fn serve_order() {}

    //     fn take_payment() {}
    // }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_wait_list();

    // Relative path
    front_of_house::hosting::add_to_wait_list();

    // Use "use"
    use crate::front_of_house::hosting;
    hosting::add_to_wait_list();
}