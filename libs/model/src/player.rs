use crate::die;
use crate::spell;
use std::vec::Vec;

pub struct Player<'a>{
    pub health: i32,
    pub energy: i32,
    pub dice: Vec<&'a mut die::Die>,
    pub spells: Vec<spell::Spell>
}

pub fn roll_dice(current_player: &mut Player) -> Vec<u32>{
    let mut res = Vec::new();
    for mut die in &mut current_player.dice {
        res.push(die::roll_die(&mut die));
    }
    return res;
}