use super::data::card_register::CardRegister;

#[derive(Clone, Copy)]
pub struct Structure {
    pub cost: i32,
    pub health: i32,
    pub defense: i32,
    pub name: &'static str,
}

pub const hill_fort: Structure = Structure {
    cost: 3,
    health: 20,
    defense: 4,
    name: "Hill Fort",
};

pub const keep: Structure = Structure {
    cost: 5,
    health: 40,
    defense: 5,
    name: "Keep",
};

pub const star_fort: Structure = Structure {
    cost: 10,
    health: 150,
    defense: 8,
    name: "Star Fort",
};
