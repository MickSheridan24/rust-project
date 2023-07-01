use super::data::card_register::CardRegister;

#[derive(Clone, Copy)]
pub struct Infra {
    pub name: &'static str,
    pub register: &'static CardRegister,
}

pub const road_building: Infra = Infra {
    name: "Fund Roardwork",
    register: &CardRegister::Roadwork,
};

pub const destroy_path: Infra = Infra {
    name: "Ruin Path",
    register: &CardRegister::DestroyPath,
};
