use rand::distributions::{Distribution, Uniform};
use std::vec::Vec;

pub struct Die{
    pub values: Vec<u32>
}

pub fn new_die(faces: u32) -> Die {
    let mut res = Die {
        values: Vec::new()
    };
    for a in 0..faces {
        res.values.push(a+1);
    }
    return res;
}

pub fn roll_die(current_die: &Die) -> u32 {
    let mut rng = rand::thread_rng();
    let uni = Uniform::from(0..current_die.values.len());

    let throw = uni.sample(&mut rng);
    return current_die.values[throw];
}