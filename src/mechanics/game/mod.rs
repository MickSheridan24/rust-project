use crate::mechanics::output::board::print_board;

use super::{
    bank::{merc_register::MercRegister, Bank},
    board::Board,
    card::{data::card_register::CardRegister, *},
    deck::Deck,
    output::*,
};
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

    pub merc_register: MercRegister,
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
            merc_register: MercRegister::new(),
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
        bank_stats(&mut self.player_bank, &self.merc_register);
        print_board(&mut self.board);
        let mut player_hand: Vec<(CardRegister, bool)> = self
            .player_deck
            .draw_hand(6)
            .iter()
            .map(|x| (*x, false))
            .collect();

        for _ in 0..3 {
            print_hand(&player_hand);

            let ops: Vec<i32> = (0..player_hand.len())
                .filter(|f| player_hand.get(*f).is_some() && player_hand.get(*f).unwrap().1)
                .map(|f| f as i32)
                .collect();

            let entry = prompt_option(ops);
            let mut card = &mut player_hand[entry as usize];
            player_play_card(&card.0.get_card());
            card.0.play(
                &EntityOwner::Player,
                &mut self.player_bank,
                &mut self.opponent_bank,
                &mut self.board,
                &mut self.merc_register,
            );
            self.discard(card.0.clone());
            card.1 = true;
        }
        player_discard_hand(&player_hand.iter().map(|x| x.0).collect());

        self.opponent_deck
            .prepend(player_hand.iter().filter(|x| !x.1).map(|x| x.0).collect());

        self.board.move_pieces(
            EntityOwner::Player,
            &mut self.player_bank,
            &mut self.opponent_bank,
            &mut self.merc_register,
        );
    }

    fn opponent_turn(&mut self) {
        println!("Opponent's Turn");
        deck_stats(&self.opponent_deck);
        bank_stats(&mut self.opponent_bank, &self.merc_register);
        print_board(&mut self.board);
        let mut opp_hand = self.opponent_deck.draw_hand(6);
        let mut rng = rand::thread_rng();
        for _ in 0..3 {
            let entry: usize = rng.gen_range(1..opp_hand.len());
            let card = &opp_hand[entry];

            opponent_play_card(card);
            card.play(
                &EntityOwner::Opponent,
                &mut self.opponent_bank,
                &mut self.player_bank,
                &mut self.board,
                &mut self.merc_register,
            );

            self.discard(card.clone());
            opp_hand.remove(entry);
        }
        self.player_deck.prepend(opp_hand);
        self.board.move_pieces(
            EntityOwner::Opponent,
            &mut self.opponent_bank,
            &mut self.player_bank,
            &mut self.merc_register,
        );
        prompt_continue();
    }

    fn discard(&mut self, card: CardRegister) {
        match card.get_card().get_card_owner() {
            EntityOwner::Player => self.player_deck.discard(card.clone()),
            EntityOwner::Opponent => self.opponent_deck.discard(card.clone()),
            EntityOwner::None => (),
        }
    }
}
