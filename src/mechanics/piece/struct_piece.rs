use crate::mechanics::card::{data::card_register::CardRegister, CardType, EntityOwner};

pub struct StructPiece {
    pub completed: i32,
    pub cost: i32,
    pub max_health: i32,
    pub remaining_health: i32,
    pub defense: i32,
    pub name: &'static str,
    pub owner: EntityOwner,
}

pub fn from_card(r: CardRegister, o: EntityOwner) -> Option<StructPiece> {
    let card = r.get_card().get_card_type();

    if let CardType::StructType(s) = card {
        return Some(StructPiece {
            completed: 1,
            cost: s.cost,
            max_health: s.health,
            remaining_health: s.health,
            defense: s.defense,
            name: s.name,
            owner: o,
        });
    }
    None
}

impl StructPiece {
    pub fn is_complete(&self) -> bool {
        self.completed >= self.cost
    }
}
