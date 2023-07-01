use self::merc_register::MercRegister;

use super::{
    board::Board,
    card::{CardType::*, EntityOwner::*},
};
use std::collections::HashMap;

use super::card::{data::card_register::CardRegister, EntityOwner};

pub mod merc_register;

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

    pub fn add_merc(
        &mut self,
        r: CardRegister,
        player: &EntityOwner,
        board: &mut Board,
        opp_bank: &Bank,
        merc_register: &mut MercRegister,
    ) {
        let bank_data = self.mercs.entry(r).and_modify(|x| *x += 1).or_insert(1);

        let card = r.get_card();

        if let MercType(merc) = card.get_card_type() {
            let mut try_betray =
                |betray_bid: &i32, betray_patron: EntityOwner, registry: &mut MercRegister| {
                    let opp_bid = opp_bank.mercs.get(&r);

                    if let Some(bid) = opp_bid {
                        if betray_bid > &bid {
                            board.betray_merc(r, betray_patron);
                            registry.add_or_change_registry(r, betray_patron);
                        }
                    }
                };
            let merc_reg = merc_register.get_registry(&r);

            match merc_reg {
                Player => {
                    if player == &Opponent {
                        try_betray(&bank_data, Opponent, merc_register);
                    }
                }
                Opponent => {
                    if player == &Player {
                        try_betray(&bank_data, Player, merc_register);
                    }
                }
                None => {
                    if &merc.cost == &bank_data {
                        board.deploy_merc(r, self.owner);
                        merc_register.add_or_change_registry(r, self.owner);
                    }
                }
            }
        }
    }

    pub fn clear_merc(&mut self, r: CardRegister, merc_register: &mut MercRegister) {
        let x = self.mercs.entry(r).and_modify(|x| *x = 0);
        merc_register.remove_registry(r);
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
