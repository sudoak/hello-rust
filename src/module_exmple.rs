mod front_of_house{
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Waitlist");
        }
        // fn seat_at_table() {
        //     println!("Seating");
        // }
    }
    pub mod serving {
        // fn take_order() {
        //     println!("Taking Order");
        // }
        // fn serve_order() {
        //     println!("Serving order");
        // }
        pub fn take_payment() {
            println!("Payment");
        }
    }
}

pub fn eating_at_cafe () {
    front_of_house::hosting::add_to_waitlist();
    front_of_house::serving::take_payment();
}