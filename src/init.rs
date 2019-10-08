
use rand::Rng;
use rand_distr::{Binomial, Distribution};

use crate::Container;
use crate::Config;

pub fn init_users(config: &Config, users: &mut Vec<Container>, range: &mut Vec<u64>) {
let mut r = rand::thread_rng();
users.clear();
for i in 0..config.n{
  users.push(Container {
    idx: i,
    p: config.prob,
    len: 1,
    obs: Vec::new(),
    retrv: false,
  });
}
for mut user in users {
  user.len = {
    let binom = Binomial::new(config.m as u64, user.p).unwrap();
    let len = binom.sample(&mut r);
    len % config.m
  };
  for i in 1..user.len + 1 {
    let rr: usize = range.len() - i as usize;
    let d: usize = r.gen();
    let d: usize = d % rr;
    let temp = range[d];
    range[d] = range[rr];
    range[rr] = temp;
    user.obs.push(temp);
  }
}
}
