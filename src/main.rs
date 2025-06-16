use std::io;

mod creature;

fn main() {
    println!("Choose your hero: Hotbabe or Toughman?");
       let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to readline");
        let mut hero: creature::Hero;
     if choice.to_lowercase().contains("toughman") {
         hero = creature::Hero::toughman();
     } else { 
         hero = creature::Hero::hotbabe();
     }
     present(&mut hero, creature::Monster::draug());
    fight(hero, creature::Monster::draug()); 
} 

pub fn this_monster(this_monster: creature::Monster) {
    let monster= this_monster;
    println!("{}! It has {}, {} and strength of {} men! Its health is {}!", monster.text, monster.hands, monster.teeth, monster.strength, monster.health)
}

pub fn this_hero(our_hero: &creature::Hero) {
    let hero = our_hero; 
    println!("Our hero walks down into the dungeons of Dascarrah! Fearless, {} searches for fortune!
    {} wears {}, {}, and has {} of strength. Remaining life is {}.",hero.gender, hero.name, hero.clothing, hero.footwear, hero.strength, hero.health)
}

pub fn present(hero: &mut creature::Hero, monster: creature::Monster) {
    this_hero(hero);
    this_monster(monster);
        println!("Choose you weapon: Sword or spear?");
       let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to readline");
        weapon(hero, choice.to_lowercase())
}

pub fn weapon(hero: &mut creature::Hero, weapon: String) {
    if hero == &mut creature::Hero::hotbabe() {
        if weapon.contains("sword") {
            hero.strength += 10
        }
        if weapon.contains("spear") {
            hero.strength += 50
        }
    } else if hero == &mut creature::Hero::toughman() {
        if weapon.contains("sword") {
            hero.strength += 30
        }
        if weapon.contains("spear") {
            hero.strength += 10
        }
    } 
    println!("Choose your power drink: potion or booze?");
       let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to readline");
        drink(hero, choice.trim().to_lowercase())
}

pub fn drink(hero: &mut creature::Hero, drink: String) {
    println!("{}", hero.name);
    if hero.name.contains("Toughman") {
        println!("Toughman is drinking {}", drink);
        if drink.contains("potion") {
            hero.strength += 10;
        } else if drink.contains("booze") {
            hero.strength -= 5 
        } else { 
            println!("Not available.")
        }
    } else if hero.name.contains("Hotbabe") {
        if drink.contains("potion") {
            hero.strength += 10
        } else if drink.contains("booze") {
            hero.strength += 30
        } else {
            println!("Not available.")
        }
    } else {
        println!("No hero selected")
}
}

pub fn fight(hero: creature::Hero, monster: creature::Monster) {
    if hero.strength >= monster.health {
        println!("The {} dies! {} wins! Your score: {}", monster.name, hero.name, hero.strength) 
    } else { 
        println!("{} dies! You lose! Your score: {}", hero.name, hero.strength)
    }
}
//TODO: Implement pet in fights. 
