pub mod agent;
pub mod equipment;
pub mod export;
pub mod infra;
pub mod merc;
pub mod spell;
pub mod structure;

use agent::*;
use equipment::*;
use export::*;
use infra::*;
use merc::*;
use spell::*;
use structure::*;

use self::data::card_register::CardRegister;

use super::{bank::Bank, board::Board};
pub mod data;

#[derive(Clone, Copy)]
pub enum CardType {
    MercType(CardRegister, Merc),
    StructType(CardRegister, Structure),
    InfraType(CardRegister, Infra),
    ExportType(CardRegister, Export),
    EquipmentType(CardRegister, Equipment),
    AgentType(CardRegister, Agent),
    SpellType(CardRegister, Spell),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntityOwner {
    Player,
    Opponent,
    None,
}

#[derive(Clone, Copy)]
pub struct Card(CardType, EntityOwner);

impl Card {
    pub fn get_card_type(&self) -> CardType {
        self.0
    }

    pub fn merc(&self) -> Option<Merc> {
        if let CardType::MercType(r, m) = self.0 {
            return Some(m);
        }
        None
    }

    pub fn get_card_owner(&self) -> EntityOwner {
        self.1
    }

    pub fn set_card_owner(&mut self, o: EntityOwner) {
        self.1 = o;
    }

    pub fn play(&self, bank: &mut Bank, board: &mut Board) {
        match self.0 {
            CardType::MercType(r, _) => bank.add_merc(r, board),
            CardType::StructType(_, _) => (),
            CardType::InfraType(_, _) => (),
            CardType::ExportType(r, _) => bank.add_export(r),
            CardType::EquipmentType(_, _) => (),
            CardType::AgentType(r, _) => bank.add_agent(r),
            CardType::SpellType(r, _) => bank.add_spell(r),
        }
    }

    pub fn get_name(&self) -> &str {
        match self.0 {
            CardType::MercType(_, m) => m.name,
            CardType::StructType(_, s) => s.name,
            CardType::InfraType(_, i) => i.name,
            CardType::ExportType(_, e) => e.name,
            CardType::EquipmentType(_, e) => e.name,
            CardType::AgentType(_, a) => a.name,
            CardType::SpellType(_, s) => s.name,
        }
    }
}
