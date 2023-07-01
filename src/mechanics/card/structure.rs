use super::data::card_register::CardRegister;

#[derive(Clone, Copy)]
pub struct Structure {
    pub cost: i8,
    pub health: i16,
    pub name: &'static str,
    pub register: &'static CardRegister,
}

pub const hill_fort: Structure = Structure {
    cost: 3,
    health: 20,
    name: "Hill Fort",
    register: &CardRegister::HillFort,
};

pub const keep: Structure = Structure {
    cost: 5,
    health: 40,
    name: "Keep",
    register: &CardRegister::Keep,
};

pub const star_fort: Structure = Structure {
    cost: 10,
    health: 150,
    name: "Star Fort",
    register: &CardRegister::StarFort,
};
