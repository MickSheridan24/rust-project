use super::card::data::card_register::CardRegister;
use super::card::data::card_register::CardRegister::*;
use super::card::EntityOwner::*;
use super::deck::*;

fn copies(r: &CardRegister, x: i8) -> Vec<CardRegister> {
    let mut v = vec![];
    for _ in 0..x {
        v.push(r.clone());
    }
    v
}

pub fn player_deck() -> Deck {
    let mut cards: Vec<CardRegister> = vec![];

    cards.append(&mut copies(&HillFolk(Opponent), 20));
    //cards.append(&mut copies(&PavisiersOfSomeRenown(Player), 20));
    cards.append(&mut copies(&HillFort(Player), 20));
    // cards.append(&mut copies( &Roadwork, 2));
    // cards.append(&mut copies( &DestroyPath, 1));
    // cards.append(&mut copies( &Wool, 3));
    // cards.append(&mut copies( &Silk, 3));
    // cards.append(&mut copies( &Spices, 3));
    // cards.append(&mut copies( &Saboteur, 2));
    // cards.append(&mut copies( &Cleric, 2));
    // cards.append(&mut copies( &CounterSpell, 3));
    // cards.append(&mut copies( &ImportedArquebuses, 1));
    // cards.append(&mut copies( &PackHorses, 3));
    // cards.append(&mut copies( &BallistaTurret, 3));

    Deck::init(cards)
}

pub fn opponent_deck() -> Deck {
    let mut cards: Vec<CardRegister> = vec![];

    cards.append(&mut copies(&SixtySevenHorsemen(Opponent), 40));
    // cards.append(&mut copies( &Keep, 5));
    // cards.append(&mut copies( &Roadwork, 2));
    // cards.append(&mut copies( &DestroyPath, 1));
    // cards.append(&mut copies( &Wool, 3));
    // cards.append(&mut copies( &Tulips, 3));
    // cards.append(&mut copies( &Timber, 3));
    // cards.append(&mut copies( &Assassin, 2));
    // cards.append(&mut copies( &Scout, 2));
    // cards.append(&mut copies( &CounterSpell, 3));
    // cards.append(&mut copies( &SeigeEngineers, 1));
    // cards.append(&mut copies( &SecretExit, 3));
    // cards.append(&mut copies( &PackHorses, 3));

    Deck::init(cards)
}
