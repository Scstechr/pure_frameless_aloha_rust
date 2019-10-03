use std::collections::HashMap;

use crate::Container;

pub fn transmit(users: &Vec<Container>, frame: &mut HashMap<u64, Vec<u64>>) {
    frame.clear();
    // println!("{:?}", frame);
    for user in users {
        for s in user.obs.iter() {
            let cur = frame.entry(*s).or_insert([].to_vec());
            cur.push(user.idx);
        }
    }
    // println!("{:?}", frame);
}

pub fn sic(users: &Vec<Container>, frame: &mut HashMap<u64, Vec<u64>>) {
    println!("{:?}", frame);
    let mut decoded: Vec<usize> = vec![0;users.len()];
    for user in users.iter() {
        println!("{:?}", user);
        for s in user.obs.iter() {
            let cur = frame.entry(*s).or_insert([].to_vec());
            if cur.len() == 1 {
                user.decoded = true;
                println!("{} <-", s);
                for s_ in user.obs.iter() {
                    let cur_ = frame.entry(*s_).or_insert([].to_vec());
                    let mut idx = 0;
                    for u in cur_.iter() {
                        if u == &user.idx {
                            break;
                        }
                        idx += 1;
                    }
                    print!("{:?}, {}, {} -> ", cur_, user.idx, idx);
                    cur_.remove(idx);
                    println!("{:?}", cur_);
                }
            }
        //     cur.push(user.idx);
        }
    }
    ;
}