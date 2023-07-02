pub mod agent;
pub mod equipment;
pub mod export;
pub mod infra;
pub mod merc;
pub mod spell;
pub mod structure;

use agent::*;
use bevy::prelude::Entity;
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
    MercType(Merc),
    StructType(Structure),
    InfraType(Infra),
    ExportType(Export),
    EquipmentType(Equipment),
    AgentType(Agent),
    SpellType(Spell),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityOwner {
    Player,
    Opponent,
    None,
}

#[derive(Clone, Copy)]
pub struct Card(CardType, EntityOwner);

impl Card {
    pub fn get_card_owner(&self) -> EntityOwner {
        self.1
    }
    pub fn get_card_type(&self) -> CardType {
        self.0
    }

    pub fn get_name(&self) -> &str {
        match self.0 {
            CardType::MercType(m) => m.name,
            CardType::StructType(s) => s.name,
            CardType::InfraType(i) => i.name,
            CardType::ExportType(e) => e.name,
            CardType::EquipmentType(e) => e.name,
            CardType::AgentType(a) => a.name,
            CardType::SpellType(s) => s.name,
        }
    }
}
