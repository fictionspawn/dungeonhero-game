pub struct Monster {
    pub name: String,
  pub hands: String,
  pub teeth: String,
  pub strength: u64,
  pub health: u64,
  pub state: bool,
  pub text: String,
}
#[derive(PartialEq)]
pub struct Hero {
   pub name: String,
   pub gender: String,
   pub clothing: String,
   pub footwear: String, 
   pub strength: u64,
   pub health: u64,
   pub state: bool,
}

pub struct Pet {
    pub name: String,
    pub species: String,
    pub strength: u64,
    pub health: u64,
    pub state: bool,
}

impl Monster {
pub fn draug() -> Monster {
let monster = Monster {
    name: String::from("draug"),
    hands: String::from("claws"),
    teeth: String::from("fangs"),
    strength: 64,
    health: 50,
    state: true,
    text: String::from("The draug is coming!"),
};
monster
}

pub fn ghoul() -> Monster {
let monster = Monster {
    name: String::from("ghoul"),
    hands: String::from("nails"),
    teeth: String::from("teeth"),
    strength: 45,
    health:  30,
    state: true,
    text: String::from("A ghoul is coming towards you!"),
};
monster
}}

impl Hero {
pub fn toughman() -> Hero {
let hero = Hero {
    name: String::from("Toughman"),
    gender: String::from("he"),
    clothing: String::from("shirt an pants"),
    footwear: String::from("boots"),
    strength: 30,
    health: 20,
    state: true,
};
hero
}
pub fn hotbabe() -> Hero {
    let hero = Hero {
        name: String::from("Hotbabe"),
        gender: String::from("she"),
        clothing: String::from( "skirt and top"),
        footwear: String::from("shoes"),
        strength: 20,
        health: 20,
        state: true,
        };
    hero
}
}

impl Pet {
    pub fn dog() -> Pet {
        let pet = Pet {
            name: String::from("Fido"),
            species: String::from("dog"),
            strength: 15,
            health: 10,
            state: true,
        };
    pet
    }
}


