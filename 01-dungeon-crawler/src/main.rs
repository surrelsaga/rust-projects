mod player;
mod monster;
// import Player and Monster types
use player::Player;
use monster::Monster;

mod action; // action types and building player/monster move
mod ui; // methods to print in terminal
mod game; // function to play one round

fn main() {
    let mut player = Player {
        hp: 10,
        attack: 4,
    };

    let mut monster = Monster {
        hp: 10,
        attack: 3,
    };

    // println! macro never take ownership of its arguments
    println!("User starts with {} hp and deals {} dmg per attack.", player.hp, player.attack);
    println!("Monster starts with {} hp and deals {} dmg per attack.", monster.hp, monster.attack);

    ui::pause_and_clear();

    while player.is_alive() && monster.is_alive() {
        // print hp of player and monster
        ui::print_hp(&player, &monster);

        // get player move
        // depends on user input, convert to an Action
        let player_move: action::Action = match action::parse_player_move(&ui::read_player_input()) {
            Some(action) => action,
            None => {
                println!("Invalid move, type attack/defend/flee.");
                continue;
            }
        };

        // get monster move (attack or defend)
        let monster_move: action::Action = action::random_monster_move();

        // start round
        game::play_game(&mut player, &mut monster, player_move, monster_move);

        // end round, wait and clear terminal
        ui::pause_and_clear();
    }

    // print the winner
    ui::print_winner(&player, &monster);
}
