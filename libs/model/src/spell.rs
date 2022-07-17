use crate::game_state;

pub struct Spell {
    pub name: String,
    pub cost: u32,
    pub description: String,
    pub action: fn(&game_state::GameState)
}