use model;
use std::vec::Vec;
use std::io;
use std::io::*;

pub fn main_loop() {
    println!("Balsamic game starting...");
    println!("Dicing the onions...");

    let mydie = model::new_die(6);
    let myspell = model::Spell { 
        name: "spell1".to_string(),
        cost: 12,
        description: "yes.".to_string(),
        action: dummy
    };

    let mydie2 = model::new_die(6);
    let myspell2 = model::Spell { 
        name: "spell1".to_string(),
        cost: 12,
        description: "yes.".to_string(),
        action: dummy
    };

    let mut state = model::GameState {
        players: [
            {   model::Player {
                health: 20,
                energy: 3,
                spells: Vec::new(),
                dice: Vec::new()
            }},
            {   model::Player {
                health: 10,
                energy: 3,
                spells: Vec::new(),
                dice: Vec::new()
            }}
        ]
    };

    state.players[0].spells.push(myspell);
    state.players[0].dice.push(mydie);
    
    state.players[1].spells.push(myspell2);
    state.players[1].dice.push(mydie2);

    // initGameState();
    println!("Salad ready! Entering main loop...");
      
    let mut input = String::new();

    while state.players[0].health > 0 && state.players[1].health > 0 {
        println!("New turn! ");
        println!("Select spell");
        io::stdin().read_line(&mut input).expect("error reading used input");

        println!("Rolling dice...");
        let results_p1 = model::roll_dice(&state.players[0]);
        let results_p2 = model::roll_dice(&state.players[1]);

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

fn dummy(_state: &model::GameState){
    println!("Dummy function called!");
}