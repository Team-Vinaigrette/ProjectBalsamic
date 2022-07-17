use crate::die;
use crate::spell;
use std::vec::Vec;

pub struct Player{
    pub health: i32,
    pub energy: i32,
    pub dice: Vec<die::Die>,
    pub spells: Vec<spell::Spell>
}

pub fn roll_dice(current_player: &Player) -> Vec<u32>{
    let mut res = Vec::new();
    for die in &current_player.dice {
        res.push(die::roll_die(&die));
    }
    return res;
}