use std::io;
use rand::Rng;

use std::{thread, time::Duration}; // to delay time

struct Player {
    hp: i32,
    attack: i32,
}

struct Monster {
    hp: i32,
    attack: i32,
}

#[derive(Debug)]
enum Action {
    Attack,
    Defend,
    Flee,
}

impl Player {
    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn take_damage(&mut self, dmg: &i32) {
        self.hp -= dmg;
    }

    fn attack(&self, another: &mut Monster) {
        another.take_damage(&self.attack);
    }

    fn attack_against_defend(&self, another: &mut Monster) {
        let reduced_damage: i32 = &self.attack / 2;
        another.take_damage(&reduced_damage);
    }
}

impl Monster {
    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn take_damage(&mut self, dmg: &i32) {
        self.hp -= dmg;
    }

    fn attack(&self, another: &mut Player) {
        another.take_damage(&self.attack);
    }

    fn attack_against_defend(&self, another: &mut Player) {
        let reduced_damage: i32 = &self.attack / 2;
        another.take_damage(&reduced_damage);
    }
}

fn parse_player_move(player_move: &str) -> Option<Action> {
    match player_move {
        "attack" => Some(Action::Attack),
        "defend" => Some(Action::Defend),
        "flee" => Some(Action::Flee),
        _ => None,
    }
}

// random function to clear terminal (w/o using external crate)
fn clear_terminal() {
    // \x1B[2J clears the screen
    // \x1B[1;1H moves the cursor to the top-left corner
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    
    // Alternatively, using standard escape syntax:
    // print!("\x1B[2J\x1B[1;1H");
}

// game logic (flee is not implemented yet since it's not straightforward)
fn play_game(player: &mut Player, monster: &mut Monster, player_move: Action, monster_move: Action) {
    // one round only

    // Only player can flee (dodge)

    // if let is just match but only have 1 case and ignore the other cases
    if let Action::Flee = player_move {

        // Flee only works if the monster is attacked
        if let Action::Attack = monster_move {
            // generate true, fasle randomly (coin flip)

            // if true then can dodge
            if rand::thread_rng().gen_bool(0.5) {
                println!("You dodged the monster's attack.");
                return; //end round
            }

            // otherwise, flee does not have any use
            // so if monster attacks, player will take damage
            println!("You failed to dodge.");
            monster.attack(player);
            return; //end round
        } 

        // otherwise
        println!("Flee against Defend so nothing happened, continue.");
        return; //end round
    }


    // handle only Attack and Defend here
    match (player_move, monster_move) {
        (Action::Attack, Action::Attack) => {
            // both attack
            player.attack(monster);
            println!("You attacked monster.");

            monster.attack(player);
            println!("Monster attacked you.");
        },

        (Action::Attack, Action::Defend) => {
            // monster takes half the damage
            player.attack_against_defend(monster);
            println!("You attacked monster, but monster defended so monster take reduced damage.");
        },

        (Action::Defend, Action::Attack) => {
            // player takes half the damage
            monster.attack_against_defend(player);
                        println!("Monster attacked you, but you defended so you take reduced damage.");
        },

        // other cases: (defend, defend) -> nothing
        _ => println!("Both defend so nothing happens."),
    }

    return; //end round if reaches here
}

fn read_player_move() -> Option<Action> {
    // TAKE USER INPUT FOR PLAYER'S MOVE

    println!("Enter your move (attack/defend/flee): ");

    let mut player_move = String::new();

    io::stdin()
        .read_line(&mut player_move)
        .expect("failed to read line");
    
    // println!("You played a {} move", player_move);

    // shadow it without a .trim() to convert to a &str 
    // so it can accept literal string
    let player_move = player_move.trim();

    // depends on user input, convert to an Action
    // also add Option<T> since the user input might not be valid so player_move can be absent
    parse_player_move(&player_move)

}

fn random_monster_move() -> Action {
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

fn print_hp(player: &Player, monster: &Monster) {
    // print out remaining hp of both player and monster
    println!("User has {} hp left.", player.hp);
    println!("Monster has {} hp left.", monster.hp);
}

fn print_winner(player: &Player, monster: &Monster) {
    if !player.is_alive() && !monster.is_alive() {
        println!("Draw! You both lose.");
    } else if !player.is_alive() {
        println!("Monster wins!");
    } else {
        println!("Player wins!");
    };
}

fn pause_and_clear() {
    thread::sleep(Duration::from_secs(2)); // delay 2 seconds before clearing
    clear_terminal();
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

    pause_and_clear();

    while player.is_alive() && monster.is_alive() {
        // print hp of player and monster
        print_hp(&player, &monster);

        // get player move
        let player_move: Action = match read_player_move() {
            Some(action) => action,
            None => {
                println!("Invalid move, type attack/defend/flee.");
                continue;
            }
        };

        // get monster move (attack or defend)
        let monster_move: Action = random_monster_move();

        // println!("{:?}", monster_move);

        // start round
        play_game(&mut player, &mut monster, player_move, monster_move);

        // end round, wait and clear terminal
        pause_and_clear();
    }

    // print the winner
    print_winner(&player, &monster);
}
