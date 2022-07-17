mod die;
pub use die::Die;
pub use die::new_die;
pub use die::roll_die;

mod spell;
pub use spell::Spell;

mod player;
pub use player::Player;
pub use player::roll_dice;

mod game_state;
pub use game_state::GameState;
pub use game_state::apply_results;