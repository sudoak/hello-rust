mod front_of_house{
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Waitlist");
        }
        fn seat_at_table() {
            println!("Seating");
        }
    }
    mod serving {
        fn take_order() {
            println!("Taking Order");
        }
        fn serve_order() {
            println!("Serving order");
        }
        fn take_payment() {
            println!("Payment");
        }
    }
}

pub fn eating_at_cafe () {
    front_of_house::hosting::add_to_waitlist();
}