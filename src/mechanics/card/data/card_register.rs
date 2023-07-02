use crate::mechanics::{
    bank::{merc_register::MercRegister, Bank},
    board::Board,
    card::{
        agent::*, equipment::*, export::*, infra::*, merc::*, spell::*, structure::*, Card,
        CardType, EntityOwner,
    },
};
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardRegister {
    MerryLandsknechts(EntityOwner),
    PavisiersOfSomeRenown(EntityOwner),
    SixtySevenHorsemen(EntityOwner),
    BloodyBuccaneers(EntityOwner),
    HillFolk(EntityOwner),
    BlackRiders(EntityOwner),

    HillFort(EntityOwner),
    Keep(EntityOwner),
    StarFort(EntityOwner),

    Roadwork(EntityOwner),
    DestroyPath(EntityOwner),

    Silk(EntityOwner),
    Wool(EntityOwner),
    Spices(EntityOwner),
    Tulips(EntityOwner),
    Timber(EntityOwner),
    Dyes(EntityOwner),

    SeigeEngineers(EntityOwner),
    ImportedArquebuses(EntityOwner),
    PackHorses(EntityOwner),

    BallistaTurret(EntityOwner),
    Harbor(EntityOwner),
    SecretExit(EntityOwner),

    Smuggler(EntityOwner),
    Cleric(EntityOwner),
    Assassin(EntityOwner),
    Saboteur(EntityOwner),
    Scout(EntityOwner),

    Cyclone(EntityOwner),
    Meteor(EntityOwner),
    PlagueRain(EntityOwner),
    LivingDead(EntityOwner),
    CounterSpell(EntityOwner),
}

impl CardRegister {
    pub fn play(
        &self,
        player: &EntityOwner,
        bank: &mut Bank,
        opp_bank: &mut Bank,
        board: &mut Board,
        mr: &mut MercRegister,
    ) {
        let card = self.get_card();

        match card.get_card_type() {
            CardType::MercType(r) => bank.add_merc(*self, player, board, opp_bank, mr),
            CardType::StructType(s) => board.handle_construction(self, *player),
            CardType::InfraType(_) => (),
            CardType::ExportType(r) => bank.add_export(*self),
            CardType::EquipmentType(_) => (),
            CardType::AgentType(r) => bank.add_agent(*self),
            CardType::SpellType(r) => bank.add_spell(*self),
        }
    }

    pub fn get_card(&self) -> Card {
        match self {
            CardRegister::MerryLandsknechts(owner) => {
                Card(CardType::MercType(merry_landsknechts), owner.clone())
            }
            CardRegister::PavisiersOfSomeRenown(owner) => {
                Card(CardType::MercType(pavisiers_of_some_renown), owner.clone())
            }
            CardRegister::SixtySevenHorsemen(owner) => {
                Card(CardType::MercType(sixty_seven_horsemen), owner.clone())
            }
            CardRegister::BloodyBuccaneers(owner) => {
                Card(CardType::MercType(bloody_buccaneers), owner.clone())
            }
            CardRegister::HillFolk(owner) => Card(CardType::MercType(hill_folk), owner.clone()),
            CardRegister::BlackRiders(owner) => {
                Card(CardType::MercType(black_riders), owner.clone())
            }

            CardRegister::HillFort(owner) => Card(CardType::StructType(hill_fort), owner.clone()),
            CardRegister::Keep(owner) => Card(CardType::StructType(keep), owner.clone()),
            CardRegister::StarFort(owner) => Card(CardType::StructType(star_fort), owner.clone()),

            CardRegister::Roadwork(owner) => {
                Card(CardType::InfraType(road_building), owner.clone())
            }
            CardRegister::DestroyPath(owner) => {
                Card(CardType::InfraType(destroy_path), owner.clone())
            }

            CardRegister::Silk(owner) => Card(CardType::ExportType(silk), owner.clone()),
            CardRegister::Wool(owner) => Card(CardType::ExportType(wool), owner.clone()),
            CardRegister::Spices(owner) => Card(CardType::ExportType(spices), owner.clone()),
            CardRegister::Tulips(owner) => Card(CardType::ExportType(tulips), owner.clone()),
            CardRegister::Timber(owner) => Card(CardType::ExportType(timber), owner.clone()),
            CardRegister::Dyes(owner) => Card(CardType::ExportType(dyes), owner.clone()),

            CardRegister::SeigeEngineers(owner) => {
                Card(CardType::EquipmentType(siege_engineers), owner.clone())
            }
            CardRegister::ImportedArquebuses(owner) => {
                Card(CardType::EquipmentType(imported_arquebuses), owner.clone())
            }
            CardRegister::PackHorses(owner) => {
                Card(CardType::EquipmentType(pack_horses), owner.clone())
            }
            CardRegister::BallistaTurret(owner) => {
                Card(CardType::EquipmentType(ballista_turret), owner.clone())
            }
            CardRegister::Harbor(owner) => Card(CardType::EquipmentType(harbor), owner.clone()),
            CardRegister::SecretExit(owner) => {
                Card(CardType::EquipmentType(secret_exit), owner.clone())
            }

            CardRegister::Smuggler(owner) => Card(CardType::AgentType(smuggler), owner.clone()),
            CardRegister::Cleric(owner) => Card(CardType::AgentType(cleric), owner.clone()),
            CardRegister::Assassin(owner) => Card(CardType::AgentType(assassin), owner.clone()),
            CardRegister::Saboteur(owner) => Card(CardType::AgentType(saboteur), owner.clone()),
            CardRegister::Scout(owner) => Card(CardType::AgentType(scout), owner.clone()),

            CardRegister::Cyclone(owner) => Card(CardType::SpellType(cyclone), owner.clone()),
            CardRegister::Meteor(owner) => Card(CardType::SpellType(meteor), owner.clone()),
            CardRegister::PlagueRain(owner) => {
                Card(CardType::SpellType(plague_rain), owner.clone())
            }
            CardRegister::LivingDead(owner) => {
                Card(CardType::SpellType(living_dead), owner.clone())
            }
            CardRegister::CounterSpell(owner) => {
                Card(CardType::SpellType(counter_spell), owner.clone())
            }
        }
    }
}
