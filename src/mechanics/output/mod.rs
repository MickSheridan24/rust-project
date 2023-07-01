use std::io::stdin;
use std::io::{self, BufRead};

pub mod attacks;
pub mod board;
use crate::mechanics::card::data::card_data::all_cards;

use super::bank::Bank;
use super::{card::Card, deck::Deck};

pub fn print_hand(hand: &Vec<Card>) {
    println!("\nYour Hand: ");
    hand.iter()
        .enumerate()
        .for_each(|(i, f)| println!("{}: {}", i, f.get_name()));
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

pub fn player_discard_hand(cards: &Vec<Card>) {
    println!("\n You discarded: ");
    cards.iter().for_each(|f| println!("{}", f.get_name()));
}

pub fn opponent_play_card(card: &Card) {
    println!("\n Opponent played {}", card.get_name());
}

pub fn deck_stats(deck: &Deck) {
    println!("Remaining: {}", deck.get_remaining().len());
    println!("Discarded: {}", deck.get_discarded().len());
}

pub fn bank_stats(bank: &mut Bank) {
    println!("\n\nBank Stats:");
    println!("Mercs:");
    let cards = all_cards();

    for (key, val) in &bank.mercs {
        let r = cards.get(&key);
        if let Some(r) = r {
            println!("  {}: {}", r.get_name(), val)
        }
    }
    println!("Exports:");
    for (key, val) in &bank.exports {
        let r = cards.get(&key);
        if let Some(r) = r {
            println!("  {}: {}", r.get_name(), val)
        }
    }

    println!("Agents:");
    for (key, val) in &bank.agents {
        let r = cards.get(&key);
        if let Some(r) = r {
            println!("  {}: {}", r.get_name(), val)
        }
    }

    println!("Spells:");
    for (key, val) in &bank.spells {
        let r = cards.get(&key);
        if let Some(r) = r {
            println!("  {}: {}", r.get_name(), val)
        }
    }
}
