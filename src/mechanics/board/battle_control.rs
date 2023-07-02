use crate::mechanics::{
    bank::{
        merc_register::{self, MercRegister},
        Bank,
    },
    card::{data::card_register::CardRegister, merc::Merc, Card},
    output::attacks::{announce_attack, announce_destruction, announce_result},
    piece::merc_piece::MercPiece,
};

use self::AttackResult::*;
use self::AttackSurvivor::*;

pub fn handle_invasion(
    must_succeed: bool,
    attacker: &mut MercPiece,
    defender: &mut MercPiece,
    player_b: &mut Bank,
    owner_b: &mut Bank,
    mr: &mut MercRegister,
) -> InvasionResult {
    let res = try_attack(attacker, defender);
    announce_result(&res);
    match res {
        Victory(survivors) | Assault(survivors) | Skirmish(survivors) | FoolsCharge(survivors) => {
            handle_survivors(&survivors, player_b, owner_b, mr);

            match survivors {
                Aggressor(_) => return InvasionResult::Success,
                Defender(_) => return InvasionResult::RemoveInvader,
                Both => {
                    if must_succeed {
                        println!("Attacker is unable to disengage and is destroyed!");
                        return InvasionResult::RemoveInvader;
                    } else {
                        return InvasionResult::Stalemate;
                    }
                }
                Neither(_, _) => return InvasionResult::RemoveBoth,
            }
        }
        Panic => {
            if must_succeed {
                println!("Attacker is unable to disengage and is destroyed!");
                return InvasionResult::RemoveInvader;
            } else {
                return InvasionResult::Stalemate;
            }
        }
    }
}

pub fn try_attack(attacker: &mut MercPiece, defender: &mut MercPiece) -> AttackResult {
    announce_attack(&attacker, &defender);

    if (attacker.strength >= defender.health) && (defender.strength < attacker.health) {
        AttackResult::Victory(AttackSurvivor::Aggressor(defender.register))
    } else if (attacker.strength >= defender.health) && (defender.strength >= attacker.health) {
        attacker.health -= 1;
        defender.health -= 2;
        let survivors = determine_survivors(&attacker, &defender);
        AttackResult::Assault(survivors)
    } else if (attacker.strength < defender.health) && (defender.strength < attacker.health) {
        attacker.health -= 1;
        defender.health -= 1;
        let survivors = determine_survivors(&attacker, &defender);
        AttackResult::Skirmish(survivors)
    } else if attacker.morale > 5 {
        defender.health -= 1;
        attacker.health = 0;
        let survivors = determine_survivors(&attacker, &defender);
        AttackResult::FoolsCharge(survivors)
    } else {
        AttackResult::Panic
    }
}

fn determine_survivors(a: &MercPiece, d: &MercPiece) -> AttackSurvivor {
    println!("ATTACKER HEALTH {}", a.health);
    println!("DEFENDER HEALTH {}", d.health);
    let a_suv: bool = a.health > 0;
    let d_suv = d.health > 0;
    if a_suv && d_suv {
        AttackSurvivor::Both
    } else if a_suv && !d_suv {
        AttackSurvivor::Aggressor(d.register)
    } else if !a_suv && d_suv {
        AttackSurvivor::Defender(a.register)
    } else {
        AttackSurvivor::Neither(a.register, d.register)
    }
}

pub fn handle_survivors(
    survivors: &AttackSurvivor,
    player_b: &mut Bank,
    owner_b: &mut Bank,
    mr: &mut MercRegister,
) {
    match survivors {
        AttackSurvivor::Aggressor(s) | AttackSurvivor::Defender(s) => {
            destroy_piece(*s, player_b, owner_b, mr)
        }
        AttackSurvivor::Both => (),
        AttackSurvivor::Neither(s, s2) => {
            destroy_piece(*s, player_b, owner_b, mr);
            destroy_piece(*s2, player_b, owner_b, mr);
        }
    };
}

#[derive(PartialEq, Eq)]

pub enum AttackSurvivor {
    Aggressor(CardRegister),
    Defender(CardRegister),
    Both,
    Neither(CardRegister, CardRegister),
}
pub enum AttackResult {
    Victory(AttackSurvivor),
    Assault(AttackSurvivor),
    Skirmish(AttackSurvivor),
    Panic,
    FoolsCharge(AttackSurvivor),
}

pub enum InvasionResult {
    RemoveInvader,
    Success,
    RemoveBoth,
    Stalemate,
}

pub fn destroy_piece(
    r: CardRegister,
    b1: &mut Bank,
    b2: &mut Bank,
    merc_register: &mut MercRegister,
) {
    b1.clear_merc(r);
    b2.clear_merc(r);
    merc_register.remove_registry(r);
    announce_destruction(&r);
}
