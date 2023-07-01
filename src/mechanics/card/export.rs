use super::data::card_register::CardRegister;

#[derive(Clone, Copy)]
pub struct Export {
    pub name: &'static str,
    pub register: &'static CardRegister,
}

pub const silk: Export = Export {
    name: "Silk",
    register: &CardRegister::Silk,
};

pub const wool: Export = Export {
    name: "Wool",
    register: &CardRegister::Wool,
};

pub const dyes: Export = Export {
    name: "Dyes",
    register: &CardRegister::Dyes,
};

pub const timber: Export = Export {
    name: "Timber",
    register: &CardRegister::Timber,
};

pub const tulips: Export = Export {
    name: "Tulips",
    register: &CardRegister::Tulips,
};

pub const spices: Export = Export {
    name: "Spices",
    register: &CardRegister::Spices,
};
