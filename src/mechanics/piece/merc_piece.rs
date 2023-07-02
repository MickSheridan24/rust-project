use crate::mechanics::card::{data::card_register::CardRegister, CardType, EntityOwner};

#[derive(Clone, Copy)]
pub struct MercPiece {
    pub betray_cost: &'static i32,
    pub health: i32,
    pub strength: i32,
    pub name: &'static str,
    pub register: CardRegister,
    pub owner: EntityOwner,
    pub morale: i32,
}

impl MercPiece {
    pub fn from_card(r: CardRegister, owner: EntityOwner) -> Option<MercPiece> {
        let c = r.get_card();
        if let CardType::MercType(m) = c.get_card_type() {
            return Some(MercPiece {
                betray_cost: m.betray_cost,
                health: *m.health,
                strength: *m.strength,
                owner,
                register: r,
                name: m.name,
                morale: 10,
            });
        }
        None
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
