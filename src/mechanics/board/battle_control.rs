use crate::mechanics::{
    output::attacks::{announce_attack, announce_result},
    piece::MercPiece,
};

pub fn try_attack(attacker: &mut MercPiece, defender: &mut MercPiece) -> AttackResult {
    announce_attack(&attacker, &defender);

    if (attacker.strength >= defender.health) && (defender.strength < attacker.health) {
        AttackResult::Victory
    } else if (attacker.strength >= defender.health) && (defender.strength >= attacker.health) {
        attacker.health -= 1;
        defender.health -= 2;
        AttackResult::Assault
    } else if (attacker.strength < defender.health) && (defender.strength < attacker.health) {
        attacker.health -= 1;
        defender.health -= 1;
        AttackResult::Skirmish
    } else if attacker.morale > 5 {
        AttackResult::FoolsCharge
    } else {
        AttackResult::Panic
    }
}

#[derive(PartialEq, Eq)]
pub enum AttackResult {
    Victory,
    Assault,
    Skirmish,
    Panic,
    FoolsCharge,
}
