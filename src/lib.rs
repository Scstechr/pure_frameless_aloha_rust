use rand_distr::{Poisson, Distribution};
use std::collections::HashMap;

const MAXFUNCSIZE :usize = 20;
const N           :u64 = 10;
const TRIAL       :u64 = 2;
// const TRIAL       :u64 = 1;
const GMAX        :u64 = 50;
const GMIN        :u64 = 0;

#[derive(Debug)]
pub struct Container {
    idx: u64,         // index on vec.
    len: u64,         // num. of rep.
    obs: Vec<u64>,
}

pub struct Config {
    n: u64,
    m: u64,
    prob: f64,
}

pub fn run() {
    println!("Hello World!");

    // Random Distributions
    let mut r = rand::thread_rng();
    let pois = Poisson::new(N as f64).unwrap();

    // Users as vector
    let mut users: Vec<Container> = Vec::new();

    // Frame as HashMap
    let mut frame: HashMap<u64, f64> = HashMap::new();

    // Simulation Parameters
    let mut config = Config {
        n: N,
        m: 1,
        prob: 1.0,
    };

    println!("#N:{}/TRIAL:{}", N, TRIAL);
    println!("G,PDR,T");
    {
        let g_ = 10.0;
        let mut rate_sum = 0.0;
        let g = g_ as f64 * 0.1;
        config.m = (config.n as f64 / g) as u64;
        for _ in 0..TRIAL
        {
            let target_degree = 1.2;
            config.prob = target_degree / config.n as f64;
            let mut range: Vec<u64> = (0..=config.m - 1).collect::<Vec<u64>>();
            println!("{:?}", range);
        }
    }
}