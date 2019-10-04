use std::collections::HashMap;

use crate::Container;

pub fn transmit(users: &Vec<Container>, frame: &mut HashMap<u64, Vec<u64>>) {
    frame.clear();
    for user in users {
        for s in user.obs.iter() {
            let cur = frame.entry(*s).or_insert([].to_vec());
            cur.push(user.idx);
        }
    }
}

fn ic(user: &Container, frame: &mut HashMap<u64, Vec<u64>>) {
    for s_ in user.obs.iter() {
        let cur_ = frame.entry(*s_).or_insert([].to_vec());
        let mut idx = 0;
        for u in cur_.iter() {
            if u == &user.idx {
                break;
            }
            idx += 1;
        }
        // if idx < cur_.len() {
        cur_.remove(idx);
        // }
    }
}

pub fn sic(users: &Vec<Container>, mut frame: &mut HashMap<u64, Vec<u64>>) -> usize{
    // println!("{:?}", frame);
    let mut decoded: Vec<usize> = vec![0;users.len()];
    let mut flag = true;
    while flag {
        flag = false;
        for (idx, user) in users.iter().enumerate() {
            if decoded[idx] == 0 {
                for s in user.obs.iter() {
                    if frame.entry(*s).or_insert([].to_vec()).len() == 1 {
                        flag = true;
                        decoded[idx] = 1;
                        ic(&user, &mut frame);
                        break;
                    }
                }
            }
        }
        // if flag {
        //     println!("{:?}, {:?}", frame, decoded);
        // }
    }
    decoded.iter().sum()
}