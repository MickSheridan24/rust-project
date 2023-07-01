use super::card::data::card_register::CardRegister;

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone)]
pub struct Deck {
    all_cards: Vec<CardRegister>,
    remaining: Vec<CardRegister>,
    discarded: Vec<CardRegister>,
}

impl Deck {
    pub fn init(cards: Vec<CardRegister>) -> Deck {
        let mut deck = Deck {
            all_cards: cards.clone(),
            remaining: cards.clone(),
            discarded: vec![],
        };
        deck.shuffle();

        deck
    }

    pub fn get_remaining(&self) -> &Vec<CardRegister> {
        &self.remaining
    }
    pub fn get_discarded(&self) -> &Vec<CardRegister> {
        &self.discarded
    }

    pub fn draw_hand(&mut self, hand_size: usize) -> Vec<CardRegister> {
        let mut hand = vec![];

        while hand.len() < hand_size {
            if self.can_draw() {
                hand.push(self.draw().unwrap())
            } else if self.discarded.len() > 0 {
                self.reset()
            } else {
                panic!("Error in Draw Hand")
            }
        }

        hand
    }

    pub fn discard(&mut self, card: CardRegister) {
        self.discarded.push(card.clone());
    }

    pub fn prepend(&mut self, cards: Vec<CardRegister>) {
        cards.iter().for_each(|f| {
            self.remaining.push(f.clone());
        })
    }

    fn reset(&mut self) {
        self.remaining = self.discarded.clone();
        self.discarded = vec![];
        self.shuffle();
    }

    fn shuffle(&mut self) {
        self.remaining.shuffle(&mut thread_rng());
    }

    fn can_draw(&self) -> bool {
        self.remaining.len() > 0
    }

    fn draw(&mut self) -> Option<CardRegister> {
        self.remaining.pop()
    }
}
