use oop_practice::oop::{hero::Hero, level_sheet, pet::Pet};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let hero = Rc::new(RefCell::new(Hero::new()));
    let level_sheet = level_sheet::LevelSheet::new();
    println!("Hero level: {}", hero.borrow().get_level());

    hero.borrow_mut().gain_exp(1100, &level_sheet);
    println!(
        "Hero level after gaining exp: {}",
        hero.borrow().get_level()
    );

    println!("STARTING PET PRACTICE");
    println!("----------------------");

    let pet = Rc::new(RefCell::new(Pet::new("Fluffy")));

    // Set the pet for the hero
    hero.borrow_mut().set_pet(Rc::clone(&pet), Rc::clone(&hero));
    pet.borrow().eat();

    // Check if the pet's owner is set correctly
    if let Some(owner) = pet.borrow().get_owner() {
        println!("Pet's owner is set.");
        println!("Owner's hp: {}", owner.borrow().get_hp());
    } else {
        println!("Pet's owner is not set.");
    };

    hero.borrow_mut().remove_pet();
    pet.borrow().eat();

    if let Some(owner) = pet.borrow().get_owner() {
        println!("Pet's owner is set.");
        println!("Owner's hp: {}", owner.borrow().get_hp());
    } else {
        println!("Pet's owner is not set.");
    };

    println!("{}", hero.borrow().get_hp());
}
