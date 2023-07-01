use std::io::stdin;
use std::io::{self, BufRead};

pub mod attacks;
pub mod board;

use super::bank::merc_register::MercRegister;
use super::bank::{merc_register, Bank};
use super::card::data::card_register::CardRegister;
use super::card::EntityOwner;
use super::{card::Card, deck::Deck};

pub fn print_hand(hand: &Vec<CardRegister>) {
    println!("\nYour Hand: ");
    hand.iter()
        .enumerate()
        .for_each(|(i, f)| println!("{}: {}", i, f.get_card().get_name()));
}

pub fn prompt_continue() {
    println!();
    println!("Press Enter to Continue...");
    println!();
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut buffer = String::new();
    let mut handle = stdin().lock();
    let _ = handle.read_line(&mut buffer);
}

pub fn prompt_hand(size: usize) -> usize {
    let mut ret: Option<usize> = None;
    while ret == None {
        println!();
        print!(">>");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        let mut buffer = String::new();
        let mut handle = stdin().lock();

        let _ = handle.read_line(&mut buffer);

        let res = buffer.trim_end().parse();

        if let Ok(r) = res {
            if r < size {
                ret = Some(r)
            } else {
                println!("Entry Not A Listed Option");
            }
        } else {
            println!("Invalid entry");
        }
    }
    ret.unwrap()
}

pub fn player_play_card(card: &Card) {
    println!("\n You played {}", card.get_name())
}

pub fn player_discard_hand(cards: &Vec<CardRegister>) {
    println!("\n You discarded: ");
    cards
        .iter()
        .for_each(|f| println!("{}", f.get_card().get_name()));
}

pub fn opponent_play_card(card: &CardRegister) {
    println!("\n Opponent played {}", card.get_card().get_name());
}

pub fn deck_stats(deck: &Deck) {
    println!("Remaining: {}", deck.get_remaining().len());
    println!("Discarded: {}", deck.get_discarded().len());
}

pub fn bank_stats(bank: &mut Bank, merc_register: &MercRegister) {
    println!("\n\nBank Stats:");
    println!("Mercs:");

    for (key, val) in &bank.mercs {
        let r = key.get_card();
        let m = merc_register.get_registry(key);
        println!("  {} ({}): {}", r.get_name(), get_owner_name(*m), val)
    }
    println!("Exports:");
    for (key, val) in &bank.exports {
        println!("  {}: {}", key.get_card().get_name(), val)
    }

    println!("Agents:");
    for (key, val) in &bank.agents {
        println!("  {}: {}", key.get_card().get_name(), val)
    }

    println!("Spells:");
    for (key, val) in &bank.spells {
        println!("  {}: {}", key.get_card().get_name(), val)
    }
}

pub fn get_owner_name(o: EntityOwner) -> &'static str {
    match o {
        EntityOwner::Player => "Player",
        EntityOwner::Opponent => "Opponent",
        EntityOwner::None => "Nobody",
    }
}
