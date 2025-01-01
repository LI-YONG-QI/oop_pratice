use super::hero::Hero;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Pet {
    name: String,
    owner: Option<Rc<RefCell<Hero>>>,
}

impl Pet {
    pub fn new(name: &str) -> Self {
        Pet {
            name: name.to_string(),
            owner: None,
        }
    }

    pub fn eat(&self) {
        println!("Pet is eating.");
        if self.owner.is_some() {
            self.get_owner().unwrap().borrow_mut().increase_hp(10);
            return;
        }
    }

    pub fn remove_owner(&mut self) {
        self.owner = None;
    }

    pub fn set_owner(&mut self, owner: Rc<RefCell<Hero>>) {
        self.owner = Some(owner);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_owner(&self) -> Option<Rc<RefCell<Hero>>> {
        self.owner.clone()
    }
}
