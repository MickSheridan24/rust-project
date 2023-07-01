use super::data::card_register::CardRegister;

#[derive(Clone, Copy)]
pub struct Spell {
    pub name: &'static str,
    pub cost: &'static i8,
}

pub const cyclone: Spell = Spell {
    name: "Cyclone",
    cost: &4,
};

pub const meteor: Spell = Spell {
    name: "Meteor",
    cost: &6,
};

pub const plague_rain: Spell = Spell {
    name: "Plague Rain",
    cost: &2,
};

pub const living_dead: Spell = Spell {
    name: "Living Dead",
    cost: &8,
};

pub const counter_spell: Spell = Spell {
    name: "Counter Spell",
    cost: &3,
};
