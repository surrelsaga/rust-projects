use std::io;

struct Player {
    hp: i32,
    attack: i32,
}

struct Monster {
    hp: i32,
    attack: i32,
}

enum Action {
    Attack,
    Defend,
    Flee,
}

// Game logic
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
}

fn main() {
    let mut player = Player {
        hp: 2,
        attack: 1,
    };

    let mut monster = Monster {
        hp: 1,
        attack: 1,
    };

    // take input
    println!("Enter your move (attack/defend/flee): ");

    let mut playerMove = String::new();

    io::stdin()
        .read_line(&mut playerMove)
        .expect("failed to read line");
    
    println!("You played a {} move", playerMove);

    // shadow it without a .trim() to convert to a &str 
    // so it can accept literal string

    let playerMove: &str = playerMove.trim();

    match playerMove {
        "attack" => player.attack(&mut monster),
        _ => println!("Invalid move."),
    }

    if !monster.is_alive() {
        println!("The monster is dead, you won.");
    }
}
