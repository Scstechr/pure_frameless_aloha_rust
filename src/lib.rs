use std::collections::HashMap;
use std::time::Instant;
use rayon::prelude::*;

pub mod init;
pub mod process;

const N           :u64 = 1000;
const TRIAL       :u64 = 1000;
// const TRIAL       :u64 = 1;
const GMAX        :u64 = 11;
const GMIN        :u64 = 10;

#[derive(Debug)]
pub struct Container {
    idx: u64,         // index on vec.
    len: u64,         // num. of rep.
    p: f64,
    obs: Vec<u64>,
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
    println!("G,PDR,T");
    for g_ in GMIN..GMAX
    {
        if g_ > 0 {
            let begin = Instant::now();
            // let g_ = 10.0;
            let g = g_ as f64 * 0.1;
            config.m = (config.n as f64 / g) as u64;
            let mut max_degree = 0.0;
            let mut max_t = 0.0;
            let mut max_pdr = 0.0;

            let mut count = 0;
            let mut range: Vec<u64> = (0..=config.m - 1).collect::<Vec<u64>>();
            // let t_ = 32;
            for t_ in 1..60 
            {
                let target_degree = t_ as f64 * 0.1;
                config.prob = target_degree / config.n as f64;
                let mut rate_sum = 0.0;
                for _ in 0..TRIAL
                {

                    init::init_users(&config, &mut users, &mut range);
                    // println!("INITIALIZE:{:?}", begin.elapsed());
                    // let begin = Instant::now();
                    process::transmit(&users, &mut frame);
                    // println!("TRANSMIT:{:?}", begin.elapsed());
                    // let begin = Instant::now();
                    let decoded = process::sic(&users, &mut frame);
                    // println!("SIC:{:?}", begin.elapsed());
                    let rate = decoded as f64 / config.n as f64;
                    rate_sum += rate;
                }
                let pdr = rate_sum / TRIAL as f64;
                let throughput = g * pdr;
                if max_t < throughput {
                    max_t = throughput;
                    max_pdr = pdr;
                    max_degree = target_degree;
                } else {
                    count += 1;
                }
                if count > 10 {
                    break;
                }
            }
            println!("{:.3},{:.1},{:.8e},{:.8}", g, max_degree, max_pdr, max_t);
            println!("Elapsed:{:?}", begin.elapsed());
        }
    }
}