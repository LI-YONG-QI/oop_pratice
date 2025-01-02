use oop_practice::oop::{hero::Hero, pet::Pet};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let hero = Rc::new(RefCell::new(Hero::new()));
    let pet = Rc::new(RefCell::new(Pet::new("Fluffy")));

    // Set the pet for the hero
    hero.borrow_mut().set_pet(Rc::clone(&pet), Rc::clone(&hero));
    pet.borrow().eat();

    // Check if the pet's owner is set correctly

    println!("Owner hp: {}", hero.borrow().get_hp());

    hero.borrow_mut().remove_pet();
    pet.borrow().eat();

    println!("Owner hp: {}", hero.borrow().get_hp());
}
