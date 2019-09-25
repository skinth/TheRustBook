mod front_of_house;

//with use we can bring a path into the scope
//use crate::front_of_house::hosting;
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    //absolute path
    //crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    //relative path
    front_of_house::hosting::add_to_waitlist();
}

use std::fmt::Result;
use std::io::Result as IoResult;