use std::io;
use rand::Rng;

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

fn parse_player_move(playerMove: &str) -> Option<Action> {
    match playerMove {
        "attack" => Some(Action::Attack),
        "defend" => Some(Action::Defend),
        "flee" => Some(Action::Flee),
        _ => None,
    }
}

// game logic (flee is not implemented yet since it's not straightforward)
fn playGame(player: &mut Player, monster: &mut Monster, playerMove: Action, monsterMove: Action) {
    // one round only

    // Only player can flee (dodge)

    // if let is just match but only have 1 case and ignore the other cases
    if let Action::Flee = playerMove {
        // generate true, fasle randomly (coin flip)

        // if true then can dodge
        if rand::thread_rng().gen_bool(0.5) {
            println!("You dodged its attack.");
        }

        // otherwise, flee does not have any use
        // so if monster attacks, player will take damage
        println!("You failed to dodge.");
        if let Action::Attack = monsterMove {
            monster.attack(player);
        }
    }


    // handle only Attack and Defend here
    match (playerMove, monsterMove) {
        (Action::Attack, Action::Attack) => {
            // both attack
            player.attack(monster);
            monster.attack(player);
        },

        (Action::Attack, Action::Defend) => {
            // monster takes half the damage
            player.attack_against_defend(monster);
        },

        (Action::Defend, Action::Attack) => {
            // player takes half the damage
            monster.attack_against_defend(player);
        },

        // other cases: (defend, defend) -> nothing
        _ => println!("Both defend so nothing happens"),
    }
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

    // TAKE USER INPUT FOR PLAYER'S MOVE

    println!("Enter your move (attack/defend/flee): ");

    let mut playerMove = String::new();

    io::stdin()
        .read_line(&mut playerMove)
        .expect("failed to read line");
    
    println!("You played a {} move", playerMove);

    // shadow it without a .trim() to convert to a &str 
    // so it can accept literal string

    // then convert to an Action (w/o expect, it would be type Option<Action> not Action)
    let playerMove: Action = parse_player_move(playerMove.trim())
                                .expect("invalid move, type attack/defend/flee");

    // RANDOMIZE MONSTER'S MOVE

    // gen number from 0 to 1
    let randomNum = rand::thread_rng().gen_range(0..=1);

    // monster Move (monster dont know how to dodge)
    let monsterMove: Action = if randomNum == 0 {
        Action::Attack
    } else {
        Action::Defend
    };

    println!("{:?}", monsterMove);


    // Test Playing 1 round (play manually)
    playGame(&mut player, &mut monster, playerMove, Action::Defend);

    println!("Monster HP after one round: {}", monster.hp);
}
