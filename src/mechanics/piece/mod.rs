use super::card::{data::card_register::CardRegister, merc::Merc, EntityOwner};

#[derive(Clone, Copy)]
pub struct MercPiece {
    pub betray_cost: &'static i32,
    pub health: i32,
    pub strength: i32,
    pub name: &'static str,
    pub register: &'static CardRegister,
    pub space: i32,
    pub owner: EntityOwner,
    pub morale: i32,
}

impl MercPiece {
    pub fn from_card(m: Merc, owner: EntityOwner, space: i32) -> MercPiece {
        MercPiece {
            betray_cost: m.betray_cost,
            health: *m.health,
            strength: *m.strength,
            space,
            owner,
            register: m.register,
            name: m.name,
            morale: 10,
        }
    }

    pub fn short_name(&self) -> String {
        let sp: Vec<String> = self
            .name
            .split(" ")
            .map(|s| s.chars().next().unwrap().to_string())
            .collect();

        sp.join("").to_uppercase()
    }
}
