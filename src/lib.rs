use std::collections::HashMap;
// use std::time::Instant;
// use rayon::prelude::*;

pub mod init;
pub mod process;

const N           :u64 = 20;
const TRIAL       :u64 = 1;
// const TRIAL       :u64 = 1;
const GMAX        :u64 = 15;
const GMIN        :u64 = 5;

#[derive(Debug)]
pub struct Container {
    idx: u64,         // index on vec.
    len: u64,         // num. of rep.
    p: f64,
    obs: Vec<u64>,
    retrv: bool,
}

pub struct Config {
    n: u64,
    m: u64,
    prob: f64,
}

pub fn run() {
    // Users as vector
    let mut users: Vec<Container> = Vec::new();

    // Frame as HashMap
    let mut frame: HashMap<u64, Vec<u64>> = HashMap::new();

    // Simulation Parameters
    let mut config = Config {
        n: N,
        m: 1,
        prob: 1.0,
    };

    println!("#N:{}/TRIAL:{}", N, TRIAL);
    println!("G,TARGET_DEGREE,PDR,T");
    let g_ = 6;
    // for g_ in GMIN..GMAX
    {
        if g_ > 0 {

            let mut max_degree = 0.0;
            let mut max_t = 0.0;
            let mut max_pdr = 0.0;
            let mut count = 0;

            let g = g_ as f64 * 0.1;
            config.m = (config.n as f64 * g) as u64;
            let mut range: Vec<u64> = (0..=config.m - 1).collect::<Vec<u64>>();

            let t = 15;
            // for t in 1..60 
            {
                let target_degree = t as f64 * 0.1;
                config.prob = target_degree / config.n as f64;
                let pdr = process::max_degree(
                    &config, &mut users, &mut frame, &mut range);
                let throughput = 1.0 / g * pdr;

                if max_t < throughput {
                    max_t = throughput;
                    max_pdr = pdr;
                    max_degree = target_degree;
                } else {
                    count += 1;
                }
                // if count > 10 {
                //     break;
                // }
            }
            println!("{:.3},{:.1},{:.8e},{:.8}", g, max_degree, max_pdr, max_t);
        }
    }
}