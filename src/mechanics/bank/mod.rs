use super::{board::Board, card::data::card_data::all_cards, output::*};
use std::collections::HashMap;

use super::card::{data::card_register::CardRegister, EntityOwner};

pub struct Bank {
    pub owner: EntityOwner,
    pub mercs: HashMap<CardRegister, i32>,
    pub exports: HashMap<CardRegister, i32>,
    pub spells: HashMap<CardRegister, i32>,
    pub agents: HashMap<CardRegister, i32>,
}

impl Bank {
    pub fn init(owner: EntityOwner) -> Bank {
        Bank {
            owner: owner,
            mercs: HashMap::new(),
            exports: HashMap::new(),
            spells: HashMap::new(),
            agents: HashMap::new(),
        }
    }

    pub fn add_merc(&mut self, r: CardRegister, board: &mut Board) {
        let count = self.mercs.entry(r).and_modify(|x| *x += 1).or_insert(1);

        let binding = all_cards();
        let m = binding.get(&r);

        if let Some(merc_card) = m {
            if let Some(merc) = merc_card.merc() {
                if merc.cost == count {
                    board.deploy_merc(merc.clone(), self.owner)
                }
            }
        }
    }

    pub fn clear_merc(&mut self, r: CardRegister) {
        self.mercs.entry(r).and_modify(|x| *x = 0);
    }

    pub fn add_export(&mut self, r: CardRegister) {
        self.exports.entry(r).and_modify(|x| *x += 1).or_insert(1);
    }

    pub fn clear_export(&mut self, r: CardRegister) {
        self.exports.entry(r).and_modify(|x| *x = 0);
    }

    pub fn add_spell(&mut self, r: CardRegister) {
        self.spells.entry(r).and_modify(|x| *x += 1).or_insert(1);
    }

    pub fn clear_spell(&mut self, r: CardRegister) {
        self.spells.entry(r).and_modify(|x| *x = 0);
    }

    pub fn add_agent(&mut self, r: CardRegister) {
        self.agents.entry(r).and_modify(|x| *x += 1).or_insert(1);
    }

    pub fn clear_agent(&mut self, r: CardRegister) {
        self.agents.entry(r).and_modify(|x| *x = 0);
    }
}
