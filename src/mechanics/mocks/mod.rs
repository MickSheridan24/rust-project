use super::card::data::card_data::all_cards;
use super::card::data::card_register::CardRegister;
use super::card::*;
use super::deck::*;

fn add_card(v: &mut Vec<Card>, r: &CardRegister, x: i8, o: EntityOwner) {
    for _ in 0..x {
        let mut c = all_cards()[r].clone();
        c.set_card_owner(o);
        v.push(c);
    }
}

fn add_player_card(v: &mut Vec<Card>, r: &CardRegister, x: i8) {
    add_card(v, r, x, EntityOwner::Player)
}

fn add_opp_card(v: &mut Vec<Card>, r: &CardRegister, x: i8) {
    add_card(v, r, x, EntityOwner::Opponent)
}

pub fn player_deck() -> Deck {
    let mut cards: Vec<Card> = vec![];

    add_player_card(&mut cards, &CardRegister::MerryLandsknechts, 20);
    add_player_card(&mut cards, &CardRegister::PavisiersOfSomeRenown, 20);
    // add_player_card(&mut cards, &CardRegister::HillFort, 5);
    // add_player_card(&mut cards, &CardRegister::Roadwork, 2);
    // add_player_card(&mut cards, &CardRegister::DestroyPath, 1);
    // add_player_card(&mut cards, &CardRegister::Wool, 3);
    // add_player_card(&mut cards, &CardRegister::Silk, 3);
    // add_player_card(&mut cards, &CardRegister::Spices, 3);
    // add_player_card(&mut cards, &CardRegister::Saboteur, 2);
    // add_player_card(&mut cards, &CardRegister::Cleric, 2);
    // add_player_card(&mut cards, &CardRegister::CounterSpell, 3);
    // add_player_card(&mut cards, &CardRegister::ImportedArquebuses, 1);
    // add_player_card(&mut cards, &CardRegister::PackHorses, 3);
    // add_player_card(&mut cards, &CardRegister::BallistaTurret, 3);

    Deck::init(cards)
}

pub fn opponent_deck() -> Deck {
    let mut cards: Vec<Card> = vec![];

    add_opp_card(&mut cards, &CardRegister::HillFolk, 20);
    add_opp_card(&mut cards, &CardRegister::SixtySevenHorsemen, 20);
    // add_opp_card(&mut cards, &CardRegister::Keep, 5);
    // add_opp_card(&mut cards, &CardRegister::Roadwork, 2);
    // add_opp_card(&mut cards, &CardRegister::DestroyPath, 1);
    // add_opp_card(&mut cards, &CardRegister::Wool, 3);
    // add_opp_card(&mut cards, &CardRegister::Tulips, 3);
    // add_opp_card(&mut cards, &CardRegister::Timber, 3);
    // add_opp_card(&mut cards, &CardRegister::Assassin, 2);
    // add_opp_card(&mut cards, &CardRegister::Scout, 2);
    // add_opp_card(&mut cards, &CardRegister::CounterSpell, 3);
    // add_opp_card(&mut cards, &CardRegister::SeigeEngineers, 1);
    // add_opp_card(&mut cards, &CardRegister::SecretExit, 3);
    // add_opp_card(&mut cards, &CardRegister::PackHorses, 3);

    Deck::init(cards)
}
