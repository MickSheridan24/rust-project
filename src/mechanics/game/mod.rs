use crate::mechanics::{bank, output::board::print_board};

use super::{bank::Bank, board::Board, card::*, deck::Deck, output::*};
use rand::Rng;
pub struct Game {
    pub player_bank: Bank,
    pub opponent_bank: Bank,

    pub player_deck: Deck,
    pub opponent_deck: Deck,

    pub player_life: bool,
    pub opponent_life: bool,

    pub player_start: bool,

    pub board: Board,
}

impl Game {
    pub fn init(pd: Deck, od: Deck) -> Game {
        Game {
            player_deck: pd,
            opponent_deck: od,
            player_life: true,
            opponent_life: true,
            player_start: Game::flip_coin(),
            player_bank: Bank::init(EntityOwner::Player),
            opponent_bank: Bank::init(EntityOwner::Opponent),
            board: Board::init_straight(),
        }
    }

    pub fn run(&mut self) -> bool {
        let mut turns: i32 = 0;
        let mut is_player_turn = self.player_start;

        while self.player_life & self.opponent_life & (turns < 300) {
            turns += 1;

            if is_player_turn {
                self.player_turn();
            } else {
                self.opponent_turn();
            }

            is_player_turn = !is_player_turn;
        }

        self.player_life
    }

    fn flip_coin() -> bool {
        let mut rng = rand::thread_rng();
        rng.gen::<bool>()
    }

    fn player_turn(&mut self) {
        println!("Player's Turn");
        deck_stats(&self.player_deck);
        bank_stats(&mut self.player_bank);
        print_board(&mut self.board);
        let mut player_hand = self.player_deck.draw_hand(6);

        for _ in 0..3 {
            print_hand(&player_hand);
            let entry = prompt_hand(player_hand.len());
            let card = &player_hand[entry];
            player_play_card(card);
            card.play(&mut self.player_bank, &mut self.board);
            self.discard(card.clone());

            player_hand.remove(entry);
        }
        player_discard_hand(&player_hand);

        self.opponent_deck.prepend(player_hand);
    }

    fn opponent_turn(&mut self) {
        println!("Opponent's Turn");
        deck_stats(&self.opponent_deck);
        bank_stats(&mut self.opponent_bank);
        let mut opp_hand = self.opponent_deck.draw_hand(6);
        let mut rng = rand::thread_rng();
        for _ in 0..3 {
            let entry: usize = rng.gen_range(1..opp_hand.len());
            let card = &opp_hand[entry];

            opponent_play_card(card);
            card.play(&mut self.opponent_bank, &mut self.board);

            self.discard(card.clone());
            opp_hand.remove(entry);
        }
        self.player_deck.prepend(opp_hand);
        prompt_continue();
    }

    fn discard(&mut self, card: Card) {
        match card.get_card_owner() {
            EntityOwner::Player => self.player_deck.discard(card.clone()),
            EntityOwner::Opponent => self.opponent_deck.discard(card.clone()),
            EntityOwner::None => (),
        }
    }
}
