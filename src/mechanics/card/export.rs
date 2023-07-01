use super::data::card_register::CardRegister;

#[derive(Clone, Copy)]
pub struct Export {
    pub name: &'static str,
}

pub const silk: Export = Export { name: "Silk" };

pub const wool: Export = Export { name: "Wool" };

pub const dyes: Export = Export { name: "Dyes" };

pub const timber: Export = Export { name: "Timber" };

pub const tulips: Export = Export { name: "Tulips" };

pub const spices: Export = Export { name: "Spices" };
