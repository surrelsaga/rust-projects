// this module contains everything about actions, logic to get player/monster move
use rand::Rng;

// All these are public so client can call

// types of Action
pub enum Action {
    Attack,
    Defend,
    Flee,
}

// take user input and turn into an Action
// the step to check validation needs the clients to do
pub fn parse_player_move(player_move: &str) -> Option<Action> {
    match player_move {
        "attack" => Some(Action::Attack),
        "defend" => Some(Action::Defend),
        "flee" => Some(Action::Flee),
        _ => None,
    }
}


// get monster move, 50% chance of getting each Action
pub fn random_monster_move() -> Action {
    // RANDOMIZE MONSTER'S MOVE

    // gen number from 0 to 1
    let random_num = rand::thread_rng().gen_range(0..=1);

    // monster Move (monster doesn't know how to dodge)
    if random_num == 0 {
        Action::Attack
    } else {
        Action::Defend
    }
}


