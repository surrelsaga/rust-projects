use crate::player::Player;
use crate::monster::Monster;
// get types of action fast
use crate::action::Action;
use Action::{Attack, Defend, Flee};

use rand::Rng; // randomize number


pub fn play_game(player: &mut Player, monster: &mut Monster, player_move: Action, monster_move: Action) {
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
