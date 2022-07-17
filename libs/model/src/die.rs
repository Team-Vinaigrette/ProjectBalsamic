use rand::distributions::{Distribution, Uniform};
use std::vec::Vec;

pub struct Die{
    pub values: Vec<u32>,
    blocked: u8,
    maxed_faces: u8,
    debuffs: u8
}

pub fn new_die(faces: u32) -> Die {
    let mut res = Die {
        values: Vec::new(),
        blocked: 0,
        maxed_faces: 0,
        debuffs: 0
    };
    for a in 0..faces {
        res.values.push(a+1);
    }
    return res;
}

pub fn roll_die(current_die: &mut Die) -> u32 {
    if current_die.blocked != 0{
        current_die.blocked -= 1;
        return 0;
    }
    let mut rng = rand::thread_rng();
    let uni = Uniform::from(0..current_die.values.len());

    let throw = uni.sample(&mut rng);
    let res = current_die.values[throw];

    if current_die.debuffs > 0 {
        for a in 0..current_die.values.len() {
            current_die.values[a] += current_die.debuffs as u32;
        }
        current_die.debuffs = 0;
    }

    return res;
}

pub fn internal_block_die(mut current_die: &mut Die){
    current_die.blocked += 1;
}

pub fn internal_max_face(mut current_die: &mut Die){
    if current_die.maxed_faces >= ((current_die.values.len() as u8) - 1){
        return;
    }

    current_die.values[current_die.maxed_faces as usize] = current_die.values.len() as u32;
    current_die.maxed_faces += 1;

    println!("Values for this die: ");
    for v in &current_die.values{
        println!("{}", v);
    }
}

pub fn internal_clear_one_face(mut current_die: &mut Die){
    if current_die.maxed_faces <= 0 {
        return;
    }

    current_die.maxed_faces -= 1;
    current_die.values[current_die.maxed_faces as usize] = (current_die.maxed_faces as u32) + 1;

    println!("Values for this die: ");
        for v in &current_die.values{
        println!("{}", v);
    }
}

pub fn internal_clear_all_faces(mut current_die: &mut Die){
    while current_die.maxed_faces > 0 {
        current_die.maxed_faces -= 1;
        current_die.values[current_die.maxed_faces as usize] = (current_die.maxed_faces as u32) + 1;
    }

    println!("Values for this die: ");
    for v in &current_die.values {
        println!("{}", v);
    }
}

pub fn internal_debuff_die(mut current_die: &mut Die){
    current_die.debuffs += 1;
    for a in 0..current_die.values.len() {
        current_die.values[a] -= 1;
    }    
    
    println!("Values for this die: ");
    for v in &current_die.values {
        println!("{}", v);
    }
}