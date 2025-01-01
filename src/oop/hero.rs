use super::level_sheet::LevelSheet;
use super::pet::Pet;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Hero {
    level: u32,
    total_exp: u32,
    hp: u32,
    pet: Option<Rc<RefCell<Pet>>>,
}

impl Hero {
    pub fn new() -> Self {
        Hero {
            level: 1,
            total_exp: 0,
            hp: 100,
            pet: None,
        }
    }

    pub fn increase_hp(&mut self, hp: u32) {
        let new_hp = self.hp + hp;
        self.set_hp(new_hp);
    }

    pub fn gain_exp(&mut self, exp: u32, level_sheet: &LevelSheet) {
        self.total_exp += exp;

        let new_level = level_sheet.query_level(self.total_exp);
        self.set_level(new_level);
    }

    pub fn remove_pet(&mut self) {
        self.get_pet().unwrap().borrow_mut().remove_owner();
        self.pet = None;
    }

    // === Public Setters === //

    pub fn set_pet(&mut self, pet: Rc<RefCell<Pet>>, self_rc: Rc<RefCell<Self>>) {
        if self.pet.is_some() {
            self.remove_pet();
        }

        pet.borrow_mut().set_owner(self_rc);
        self.pet = Some(pet);
    }

    // === Getters === //

    pub fn get_level(&self) -> u32 {
        self.level
    }

    pub fn get_pet(&self) -> Option<Rc<RefCell<Pet>>> {
        self.pet.clone()
    }

    pub fn get_hp(&self) -> u32 {
        self.hp
    }

    fn set_level(&mut self, new_level: u32) {
        self.level = new_level;
    }

    fn set_hp(&mut self, hp: u32) {
        self.hp = hp;
    }
}
