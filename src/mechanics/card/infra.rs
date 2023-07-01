use super::data::card_register::CardRegister;

#[derive(Clone, Copy)]
pub struct Infra {
    pub name: &'static str,
}

pub const road_building: Infra = Infra {
    name: "Fund Roardwork",
};

pub const destroy_path: Infra = Infra { name: "Ruin Path" };
