use model;
use std::vec::Vec;
use std::io;

pub fn main_loop() {
    println!("Balsamic game starting...");
    println!("Dicing the onions...");


    let mut state = init_game_state();
    println!("Salad ready! Seasoning...");
      
    let mut die1 = model::new_die(10);
    state.players[0].dice.push(&mut die1);
    let mut die2 = model::new_die(20);
    state.players[0].dice.push(&mut die2);
    let mut die3 = model::new_die(6);
    state.players[0].dice.push(&mut die3);
    let mut die4 = model::new_die(6);
    state.players[0].dice.push(&mut die4);
    let mut die5 = model::new_die(6);
    state.players[0].dice.push(&mut die5);
    let mut die6 = model::new_die(6);
    state.players[0].dice.push(&mut die6);

    let mut die11 = model::new_die(10);
    state.players[1].dice.push(&mut die11);
    let mut die12 = model::new_die(20);
    state.players[1].dice.push(&mut die12);
    let mut die13 = model::new_die(6);
    state.players[1].dice.push(&mut die13);
    let mut die14 = model::new_die(6);
    state.players[1].dice.push(&mut die14);
    let mut die15 = model::new_die(6);
    state.players[1].dice.push(&mut die15);
    let mut die16 = model::new_die(6);
    state.players[1].dice.push(&mut die16);
    
    let mut input = String::new();

    println!("Ready to eat! Entering main loop");
    while state.players[0].health > 0 && state.players[1].health > 0 {
        println!("New turn! ");
        println!("Select spell");
        io::stdin().read_line(&mut input).expect("error reading used input");

        println!("Rolling dice...");
        let results_p1 = model::roll_dice(&mut state.players[0]);
        let results_p2 = model::roll_dice(&mut state.players[1]);

        io::stdin().read_line(&mut input).expect("error reading used input");
        println!("The rolls have been rollingly rolled...");        
        let sum_p1: u32 = results_p1.iter().sum();
        let sum_p2: u32 = results_p2.iter().sum();
        println!("P1: {}, P2: {}", sum_p1, sum_p2);
        model::apply_results(&mut state, results_p1, results_p2);
        
        io::stdin().read_line(&mut input).expect("error reading used input");
    }

    println!("Exiting main loop");
}

fn init_game_state<'a>() -> model::GameState<'a> {
    println!("Dicing the onions");

    let mut state = model::GameState {
        players: [
            {   model::Player {
                health: 100,
                energy: 3,
                spells: Vec::new(),
                dice: Vec::new()
            }},
            {   model::Player {
                health: 100,
                energy: 3,
                spells: Vec::new(),
                dice: Vec::new()
            }}
        ]
    };

    for player in 0..1 {
        let block_spell = model::Spell { 
            name: "Block die".to_string(),
            cost: 1,
            description: "Blocks one enemy die for the turn".to_string(),
            action: model::block_die_spell
        };
        let max_single_face_spell = model::Spell { 
            name: "Max one face".to_string(),
            cost: 1,
            description: "Sets the lowest face on the chosen die to the highest face value of this die".to_string(),
            action: model::max_face_spell
        };
        let max_all_dice_spell = model::Spell { 
            name: "Max one face on all dice".to_string(),
            cost: 3,
            description: "Sets the lowest face on all dice to the highest possible value".to_string(),
            action: model::max_all_faces_spell
        };
        let clear_spell = model::Spell { 
            name: "Clear face".to_string(),
            cost: 1,
            description: "Removes face buffs on one of the enemy's dice's face".to_string(),
            action: model::clear_face_spell
        };
        let clear_die_spell = model::Spell { 
            name: "Clear die".to_string(),
            cost: 2,
            description: "Removes face buffs on all faces of an enemy die".to_string(),
            action: model::clear_all_faces_spell
        };
        let clear_dice_spell = model::Spell { 
            name: "Clear all dice".to_string(),
            cost: 4,
            description: "Removes face buffs on all of the enemy's dice's".to_string(),
            action: model::clear_all_dice_spell
        };
        let debuff_spell = model::Spell { 
            name: "Debuff opponent dice".to_string(),
            cost: 2,
            description: "Lowers all dice faces of the opponent by 1 for next turn".to_string(),
            action: model::debuff_all_dice_spell
        };

        state.players[player].spells.push(block_spell);
        state.players[player].spells.push(max_single_face_spell);
        state.players[player].spells.push(max_all_dice_spell);
        state.players[player].spells.push(clear_spell);
        state.players[player].spells.push(clear_die_spell);
        state.players[player].spells.push(clear_dice_spell);
        state.players[player].spells.push(debuff_spell);
    }

    return state;
}