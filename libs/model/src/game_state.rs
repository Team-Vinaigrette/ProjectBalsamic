use crate::player;

pub struct GameState {
    pub players: [player::Player; 2]
}

pub fn apply_results(mut current_state: &mut GameState, results_p1: Vec<u32>, results_p2: Vec<u32>){
    let sum_p1: u32 = results_p1.iter().sum();
    let sum_p2: u32 = results_p2.iter().sum();

    if sum_p1 > sum_p2 {
        current_state.players[1].health -= sum_p1 as i32 - sum_p2 as i32;
    }
    else if sum_p1 < sum_p2 {
        current_state.players[0].health -= sum_p2 as i32 - sum_p1 as i32;
    }
}