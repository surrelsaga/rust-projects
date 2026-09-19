// main is the crate root is when export it doesn't have to specify crate
use crate::player::Player;

pub struct Monster {
    pub hp: i32,
    pub attack: i32,
}

impl Monster {
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn take_damage(&mut self, dmg: &i32) {
        self.hp -= dmg;
    }

    pub fn attack(&self, another: &mut Player) {
        another.take_damage(&self.attack);
    }

    pub fn attack_against_defend(&self, another: &mut Player) {
        let reduced_damage: i32 = &self.attack / 2;
        another.take_damage(&reduced_damage);
    }
}
