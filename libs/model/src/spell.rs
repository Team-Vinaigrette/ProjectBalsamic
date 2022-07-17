use crate::game_state;
use crate::die;

pub struct Spell {
    pub name: String,
    pub cost: u32,
    pub description: String,
    pub action: fn(&mut game_state::GameState, u32)
}

fn ask_die_id_stub() -> u32 {
    return 0;
}

pub fn block_die_spell(state: &mut game_state::GameState, player_id: u32){
    let die_id = ask_die_id_stub();
    let play_id: usize = player_id as usize;

    // println!("Blocking die {} for player {}", die_id, play_id);

    die::internal_block_die(&mut state.players[1-play_id as usize].dice[die_id as usize]);
}

pub fn max_face_spell(state: &mut game_state::GameState, player_id: u32){
    let die_id = ask_die_id_stub();
    let play_id: usize = player_id as usize;

    // println!("Maxing face on die {} for player {}", die_id, play_id);

    die::internal_max_face(&mut state.players[play_id as usize].dice[die_id as usize]);
}

pub fn max_all_faces_spell(state: &mut game_state::GameState, player_id: u32){
    let play_id: usize = player_id as usize;

    // println!("Maxing a face on all die for player {}", play_id);

    for mut die_it in &mut state.players[play_id].dice {
        die::internal_max_face(&mut die_it);
    }
}

pub fn clear_face_spell(state: &mut game_state::GameState, player_id: u32){
    let die_id = ask_die_id_stub();
    let play_id: usize = player_id as usize;

    // println!("Clearing a face on die {} for player {}", die_id, play_id);

    die::internal_clear_one_face(&mut state.players[1-play_id as usize].dice[die_id as usize]);
}

pub fn clear_all_faces_spell(state: &mut game_state::GameState, player_id: u32){
    let die_id = ask_die_id_stub();
    let play_id: usize = player_id as usize;

    // println!("Clearing a face on die {} for player {}", die_id, play_id);

    die::internal_clear_one_face(&mut state.players[1-play_id as usize].dice[die_id as usize]);
}

pub fn clear_all_dice_spell(state: &mut game_state::GameState, player_id: u32){
    let play_id: usize = player_id as usize;

    // println!("Maxing a face on all die for player {}", play_id);

    for mut die_it in &mut state.players[1 - play_id as usize].dice {
        die::internal_clear_all_faces(&mut die_it);
    }
}

pub fn debuff_all_dice_spell(state: &mut game_state::GameState, player_id: u32){
    let play_id: usize = player_id as usize;

    // println!("Reducing all faces on enemy dice by 1 for a turn");

    for mut die_it in &mut state.players[1 - play_id as usize].dice {
        die::internal_debuff_die(&mut die_it);
    }
}