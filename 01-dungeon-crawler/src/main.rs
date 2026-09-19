use rand::Rng;

mod player;
mod monster;
// import Player and Monster types
use player::Player;
use monster::Monster;

mod action;
// get types of action fast
use action::Action;
use Action::{Attack, Defend, Flee};

// import some methods to print in terminal
mod ui;

// game logic (flee is not implemented yet since it's not straightforward)
fn play_game(player: &mut Player, monster: &mut Monster, player_move: Action, monster_move: Action) {
    // one round only

    // Only player can flee (dodge)

    // if let is just match but only have 1 case and ignore the other cases
    if let Flee = player_move {

        // Flee only works if the monster is attacked
        if let Attack = monster_move {
            // generate true, fasle randomly (coin flip)

            // if true then can dodge
            if rand::thread_rng().gen_bool(0.5) {
                println!("You dodged the monster's attack.");
                return; //end round
            }

            // otherwise, flee does not have any use
            // so if monster attacks, player will take damage
            println!("You failed to dodge, and received full dmg from the monster.");
            monster.attack(player);
            return; //end round
        } 

        // otherwise
        println!("Flee against Defend so nothing happened, continue.");
        return; //end round
    }


    // handle only Attack and Defend here
    match (player_move, monster_move) {
        (Attack, Attack) => {
            // both attack
            player.attack(monster);
            println!("You attacked monster.");

            monster.attack(player);
            println!("Monster attacked you.");
        },

        (Attack, Defend) => {
            // monster takes half the damage
            player.attack_against_defend(monster);
            println!("You attacked monster, but monster defended so monster take reduced damage.");
        },

        (Defend, Attack) => {
            // player takes half the damage
            monster.attack_against_defend(player);
                        println!("Monster attacked you, but you defended so you take reduced damage.");
        },

        // other cases: (defend, defend) -> nothing
        _ => println!("Both defend so nothing happens."),
    }

    return; //end round if reaches here
}

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
        let player_move: Action = match action::parse_player_move(&ui::read_player_input()) {
            Some(action) => action,
            None => {
                println!("Invalid move, type attack/defend/flee.");
                continue;
            }
        };

        // get monster move (attack or defend)
        let monster_move: Action = action::random_monster_move();

        // start round
        play_game(&mut player, &mut monster, player_move, monster_move);

        // end round, wait and clear terminal
        ui::pause_and_clear();
    }

    // print the winner
    ui::print_winner(&player, &monster);
}
