use std::collections::HashMap;

use crate::Container;
use crate::Config;
use crate::init;
use crate::TRIAL;

fn transmit(users: &Vec<Container>, frame: &mut HashMap<u64, Vec<u64>>) {
    frame.clear();
    for user in users {
        for s in user.obs.iter() {
            let cur = frame.entry(*s).or_insert([].to_vec());
            cur.push(user.idx);
        }
    }
}

fn ic(user: &mut Container, frame: &mut HashMap<u64, Vec<u64>>) {
    user.retrv = true;
    for s in user.obs.iter() {
        let cur = frame.entry(*s).or_insert([].to_vec());
        let idx = cur.iter().position(|&x| x == user.idx).unwrap();
        cur.remove(idx);
    }
}

fn sic(users: &mut Vec<Container>, mut frame: &mut HashMap<u64, Vec<u64>>) -> usize{
    let mut flag = true;
    let mut count = 0;
    println!("{:?}, {}", frame, count);
    while flag {
        flag = false;
        for mut user in users.iter_mut() {
            if !user.retrv {
                for s in user.obs.iter() {
                    if frame.entry(*s).or_insert([].to_vec()).len() == 1 {
                        flag = true;
                        count += 1;
                        ic(&mut user, &mut frame);
                        break;
                    }
                }
            }
        }
        if flag {
            println!("{:?}, {}", frame, count);
        }
    }
    count
}

pub fn max_degree(config: &Config, mut users: &mut Vec<Container>, mut frame: &mut HashMap<u64, Vec<u64>>, mut range: &mut Vec<u64>) -> f64{
    let mut rate_sum = 0.0;
    for _ in 0..TRIAL
    {
        init::init_users(&config, &mut users, &mut range);
        transmit(&users, &mut frame);
        let decoded = sic(&mut users, &mut frame);
        let rate = decoded as f64 / config.n as f64;
        rate_sum += rate;
        // rate_sum += process::proc_loop(&config, &mut users, &mut frame, &mut range);
    }
    rate_sum
}