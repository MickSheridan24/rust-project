use crate::mechanics::{
    card::{data::card_register::CardRegister, EntityOwner},
    output::{board::print_board_construction, prompt_option},
};

use super::Board;

impl Board {
    pub fn handle_construction(&mut self, c: &CardRegister, o: EntityOwner) {
        match o {
            EntityOwner::Player => self.handle_construction_player(c),
            EntityOwner::Opponent => self.handle_construction_ai(c),
            EntityOwner::None => (),
        }
    }

    fn handle_construction_player(&self, c: &CardRegister) {
        print_board_construction(self, EntityOwner::Player);
        let options: Vec<i32> = self
            .ownership
            .iter()
            .filter(|x| x.1 == &EntityOwner::Player)
            .filter(|x| {
                self.structures.get(x.0).is_none()
                    || !self.structures.get(x.0).unwrap().is_complete()
            })
            .map(|f| *f.0)
            .collect();

        let entry = prompt_option(options);
    }

    fn handle_construction_ai(&self, c: &CardRegister) {}
}
