use oop_practice::oop::{hero::Hero, level_sheet};

fn main() {
    let mut hero = Hero::new();
    let level_sheet = level_sheet::LevelSheet::new();
    println!("Hero value: {}", hero.get_level());

    hero.gain_exp(1100, &level_sheet);
    println!("Hero value: {}", hero.get_level());
}
