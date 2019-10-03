use std::collections::HashMap;

use crate::Container;

pub fn transmit(users: &Vec<Container>, frame: &mut HashMap<u64, Vec<u64>>) {
    frame.clear();
    println!("{:?}", frame);
    for user in users {
        println!("{:?}", user);
        for s in user.obs.iter() {
            ;
        }
    }
}