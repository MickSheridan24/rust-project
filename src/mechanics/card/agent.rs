use super::data::card_register::CardRegister;

#[derive(Clone, Copy)]
pub struct Agent {
    pub name: &'static str,
    pub cost: &'static i8,
    pub betray_cost: &'static i8,
    pub register: &'static CardRegister,
}

pub const smuggler: Agent = Agent {
    name: "Smuggler",
    cost: &2,
    betray_cost: &2,
    register: &CardRegister::Smuggler,
};

pub const cleric: Agent = Agent {
    name: "cleric",
    cost: &5,
    betray_cost: &4,
    register: &CardRegister::Cleric,
};
pub const saboteur: Agent = Agent {
    name: "Saboteur",
    cost: &3,
    betray_cost: &2,
    register: &CardRegister::Saboteur,
};

pub const assassin: Agent = Agent {
    name: "Assassin",
    cost: &4,
    betray_cost: &2,
    register: &CardRegister::Assassin,
};

pub const scout: Agent = Agent {
    name: "Scout",
    cost: &2,
    betray_cost: &3,
    register: &CardRegister::Scout,
};
