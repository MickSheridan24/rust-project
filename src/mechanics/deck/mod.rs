use super::card::*;

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone)]
pub struct Deck {
    all_cards: Vec<Card>,
    remaining: Vec<Card>,
    discarded: Vec<Card>,
}

impl Deck {
    pub fn init(cards: Vec<Card>) -> Deck {
        let mut deck = Deck {
            all_cards: cards.clone(),
            remaining: cards.clone(),
            discarded: vec![],
        };
        deck.shuffle();

        deck
    }

    pub fn get_remaining(&self) -> &Vec<Card> {
        &self.remaining
    }
    pub fn get_discarded(&self) -> &Vec<Card> {
        &self.discarded
    }

    pub fn draw_hand(&mut self, hand_size: usize) -> Vec<Card> {
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

    pub fn discard(&mut self, card: Card) {
        self.discarded.push(card.clone());
    }

    pub fn prepend(&mut self, cards: Vec<Card>) {
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

    fn draw(&mut self) -> Option<Card> {
        self.remaining.pop()
    }
}
