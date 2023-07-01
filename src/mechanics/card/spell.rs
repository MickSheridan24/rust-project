use super::data::card_register::CardRegister;

#[derive(Clone, Copy)]
pub struct Spell {
    pub name: &'static str,
    pub cost: &'static i8,
    pub register: &'static CardRegister,
}

pub const cyclone: Spell = Spell {
    name: "Cyclone",
    cost: &4,
    register: &CardRegister::Cyclone,
};

pub const meteor: Spell = Spell {
    name: "Meteor",
    cost: &6,
    register: &CardRegister::Meteor,
};

pub const plague_rain: Spell = Spell {
    name: "Plague Rain",
    cost: &2,
    register: &CardRegister::PlagueRain,
};

pub const living_dead: Spell = Spell {
    name: "Living Dead",
    cost: &8,
    register: &CardRegister::LivingDead,
};

pub const counter_spell: Spell = Spell {
    name: "Counter Spell",
    cost: &3,
    register: &CardRegister::CounterSpell,
};
