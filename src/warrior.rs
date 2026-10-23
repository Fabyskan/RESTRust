#[derive(Debug)]
pub struct Warrior {
    name: String,
    hp: u8,
    armor: u8,
    attack: u8,
}

impl Warrior {
    pub fn new(name: String, hp: u8, armor: u8, attack: u8) -> Warrior {
        Warrior { name, hp, armor, attack }
    }
        pub fn name(&self) -> &str {
        &self.name
    }
    pub fn hp(&self) -> u8 {
        self.hp
    }
    pub fn armor(&self) -> u8 {
        self.armor
    }
    pub fn attack(&self) -> u8 {
        self.attack
    }
    pub fn warrior_print(&self) {
        println!("{:#?}", self)
    }
}
