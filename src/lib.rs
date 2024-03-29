use std::fs::File;
use std::io::{LineWriter, Write};
use std::collections::HashMap;

pub mod init;
pub mod process;

const N           :u64 = 100;
const TRIAL       :u64 = 100;
// const TRIAL       :u64 = 1;
const GMAX        :u64 = 14;
const GMIN        :u64 = 2;

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

pub fn run(output: &String) {
    // Output
    let mut f = LineWriter::new(File::create(output).unwrap());

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

    let status = format!("#N:{}|trial:{}\n", N, TRIAL);
    print!("{}", status);
    println!("G,TARGET_DEGREE,PDR,T");
    // let g_ = 6;
    let mut last_degree = 10;
    for g_ in GMIN..GMAX
    {
        if g_ > 0 {

            let mut max_degree = 0.0;
            let mut max_t = 0.0;
            let mut max_pdr = 0.0;
            let mut count = 0;

            let g = g_ as f64 * 0.1;
            config.m = (config.n as f64 / g) as u64;
            let mut range: Vec<u64> = (0..=config.m - 1).collect::<Vec<u64>>();

            // let t = 15;
            for t in last_degree..60
            {
                let target_degree = t as f64 * 0.1;
                config.prob = target_degree / config.n as f64;
                let pdr = process::max_degree(
                    &config, &mut users, &mut frame, &mut range);
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
            last_degree = (max_degree * 10.0) as usize - 10;
            if last_degree < 10 {
                last_degree = 10;
            }
            let w = format!("{:.3},{:.1},{:.8e},{:.8}\n", g, max_degree, max_pdr, max_t);
            f.write(w.as_bytes()).unwrap();
            print!("{}", w);
        }
    }
}