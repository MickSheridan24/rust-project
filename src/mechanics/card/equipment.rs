use super::data::card_register::CardRegister;

#[derive(Clone, Copy)]

pub enum EquipmentType {
    Merc,
    Structure,
    Agent,
}

#[derive(Clone, Copy)]
pub struct Equipment {
    pub name: &'static str,
    pub eq_type: &'static EquipmentType,
    pub register: &'static CardRegister,
}

pub const siege_engineers: Equipment = Equipment {
    name: "Siege Engineers",
    eq_type: &EquipmentType::Merc,
    register: &CardRegister::SeigeEngineers,
};

pub const imported_arquebuses: Equipment = Equipment {
    name: "Imported Arquebuses",
    eq_type: &EquipmentType::Merc,
    register: &CardRegister::ImportedArquebuses,
};

pub const pack_horses: Equipment = Equipment {
    name: "Pack Horses",
    eq_type: &EquipmentType::Merc,
    register: &CardRegister::PackHorses,
};

pub const ballista_turret: Equipment = Equipment {
    name: "Ballista Turret",
    eq_type: &EquipmentType::Structure,
    register: &CardRegister::BallistaTurret,
};

pub const harbor: Equipment = Equipment {
    name: "Harbor",
    eq_type: &EquipmentType::Structure,
    register: &CardRegister::Harbor,
};

pub const secret_exit: Equipment = Equipment {
    name: "Secret Exit",
    eq_type: &EquipmentType::Structure,
    register: &CardRegister::SecretExit,
};
