use crate::monster::Monster;

pub struct Player {
    pub hp: i32,
    pub attack: i32,
}

impl Player {
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn take_damage(&mut self, dmg: &i32) {
        self.hp -= dmg;
    }

    pub fn attack(&self, another: &mut Monster) {
        another.take_damage(&self.attack);
    }

    pub fn attack_against_defend(&self, another: &mut Monster) {
        let reduced_damage: i32 = &self.attack / 2;
        another.take_damage(&reduced_damage);
    }
}
