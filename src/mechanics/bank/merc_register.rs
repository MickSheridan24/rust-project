use hashbrown::HashMap;

use crate::mechanics::card::{data::card_register::CardRegister, EntityOwner};

pub struct MercRegister(HashMap<CardRegister, EntityOwner>);

impl MercRegister {
    pub fn new() -> MercRegister {
        MercRegister(HashMap::new())
    }

    pub fn add_or_change_registry(&mut self, r: CardRegister, o: EntityOwner) {
        let mut owner = self.0.entry(r).or_insert(o);
        *owner = o;
    }
    pub fn remove_registry(&mut self, r: CardRegister) {
        self.add_or_change_registry(r, EntityOwner::None);
    }

    pub fn get_registry(&self, r: &CardRegister) -> &EntityOwner {
        match self.0.get(r) {
            Some(r) => r,
            None => &EntityOwner::None,
        }
    }
}
