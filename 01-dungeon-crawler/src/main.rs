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
    // hardcoded gameplay first
    let mut player = Player {
        hp: 2,
        attack: 1,
    };

    let mut monster = Monster {
        hp: 1,
        attack: 1,
    };

    player.attack(&mut monster);

    if !monster.is_alive() {
        println!("The monster is dead, we won.");
    }
}
