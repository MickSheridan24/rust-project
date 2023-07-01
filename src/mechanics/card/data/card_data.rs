use crate::mechanics::card::*;
use std::collections::HashMap;

pub fn all_cards() -> HashMap<CardRegister, Card> {
    let mut h = HashMap::<CardRegister, Card>::new();

    vec![
        merry_landsknechts,
        pavisiers_of_some_renown,
        sixty_seven_horsemen,
        black_riders,
        bloody_buccaneers,
        hill_folk,
    ]
    .iter()
    .for_each(|m| add_merc(&mut h, *m));

    vec![hill_fort, keep, star_fort]
        .iter()
        .for_each(|f| add_structure(&mut h, *f));

    vec![silk, wool, dyes, timber, spices, tulips]
        .iter()
        .for_each(|e| add_export(&mut h, *e));

    vec![road_building, destroy_path]
        .iter()
        .for_each(|r| add_infra(&mut h, *r));

    vec![
        pack_horses,
        imported_arquebuses,
        ballista_turret,
        secret_exit,
        harbor,
        siege_engineers,
    ]
    .iter()
    .for_each(|e| add_equipment(&mut h, *e));

    vec![smuggler, assassin, saboteur, cleric, scout]
        .iter()
        .for_each(|a| add_agent(&mut h, *a));

    vec![cyclone, plague_rain, meteor, living_dead, counter_spell]
        .iter()
        .for_each(|s| add_spell(&mut h, *s));

    h
}

fn add_merc(h: &mut HashMap<CardRegister, Card>, m: Merc) {
    h.insert(
        m.register.clone(),
        Card(CardType::MercType(m.register.clone(), m), EntityOwner::None),
    );
}

fn add_structure(h: &mut HashMap<CardRegister, Card>, s: Structure) {
    h.insert(
        s.register.clone(),
        Card(
            CardType::StructType(s.register.clone(), s),
            EntityOwner::None,
        ),
    );
}
fn add_export(h: &mut HashMap<CardRegister, Card>, s: Export) {
    h.insert(
        s.register.clone(),
        Card(
            CardType::ExportType(s.register.clone(), s),
            EntityOwner::None,
        ),
    );
}

fn add_infra(h: &mut HashMap<CardRegister, Card>, s: Infra) {
    h.insert(
        s.register.clone(),
        Card(
            CardType::InfraType(s.register.clone(), s),
            EntityOwner::None,
        ),
    );
}

fn add_equipment(h: &mut HashMap<CardRegister, Card>, s: Equipment) {
    h.insert(
        s.register.clone(),
        Card(
            CardType::EquipmentType(s.register.clone(), s),
            EntityOwner::None,
        ),
    );
}

fn add_agent(h: &mut HashMap<CardRegister, Card>, s: Agent) {
    h.insert(
        s.register.clone(),
        Card(
            CardType::AgentType(s.register.clone(), s),
            EntityOwner::None,
        ),
    );
}

fn add_spell(h: &mut HashMap<CardRegister, Card>, s: Spell) {
    h.insert(
        s.register.clone(),
        Card(
            CardType::SpellType(s.register.clone(), s),
            EntityOwner::None,
        ),
    );
}
