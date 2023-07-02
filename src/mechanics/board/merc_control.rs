use std::borrow::BorrowMut;

use crate::mechanics::{
    bank::{merc_register::MercRegister, Bank},
    board::battle_control::InvasionResult,
    card::{
        data::card_register::CardRegister,
        EntityOwner::{self, *},
    },
    output::attacks::{announce_betrayal, announce_deployment},
    piece::merc_piece::MercPiece,
};

use super::{
    battle_control::{handle_invasion, InvasionResult::*},
    Board,
};

impl Board {
    pub fn deploy_merc(
        &mut self,
        r: CardRegister,
        owner: EntityOwner,
        player_b: &mut Bank,
        owner_b: &mut Bank,
        mr: &mut MercRegister,
    ) {
        let space = match owner {
            Player => self.player_start,
            Opponent => self.opponent_start,
            None => -1,
        };
        let mut piece_res = MercPiece::from_card(r, owner);
        if let Some(piece) = piece_res.borrow_mut() {
            announce_deployment(&piece, owner);
            self.deploy_or_oust(piece, space, player_b, owner_b, mr);
        }
    }

    pub fn deploy_or_oust(
        &mut self,
        piece: &mut MercPiece,
        space: i32,
        player_b: &mut Bank,
        opponent_b: &mut Bank,
        mr: &mut MercRegister,
    ) {
        let mut ex: Option<MercPiece> = Option::None;

        let existing_op = self.mercs.get_mut(&space);
        match existing_op {
            Some(existing) => {
                if existing.owner == piece.owner {
                    ex = Some(*existing);
                    self.mercs.entry(space).and_modify(|x| *x = *piece);
                } else {
                    match handle_invasion(true, piece, existing, player_b, opponent_b, mr) {
                        RemoveInvader => {}
                        Success => {
                            self.mercs.remove(&space);
                            self.mercs.insert(space, *piece);
                            self.ownership.entry(space).and_modify(|x| *x = piece.owner);
                        }
                        RemoveBoth => {
                            self.mercs.remove(&space);
                        }
                        Stalemate => {}
                    };
                }
            }
            Option::None => {
                self.mercs.insert(space, *piece);
                self.ownership.entry(space).and_modify(|x| *x = piece.owner);
            }
        }

        if let Some(existing) = ex.as_mut() {
            let next_space = match piece.owner {
                EntityOwner::Player => space + 1,
                EntityOwner::Opponent => space - 1,
                EntityOwner::None => panic!("piece has no owner"),
            };
            self.deploy_or_oust(existing, next_space, player_b, opponent_b, mr);
        }
    }

    pub fn betray_merc(&mut self, r: CardRegister, o: EntityOwner) {
        let mut space: Option<&i32> = Option::None;
        let mut piece: Option<&mut MercPiece> = Option::None;

        for (k, merc) in &mut self.mercs {
            if merc.register == r {
                space = Some(k);
                piece = Some(merc);
            }
        }

        if let Some(sp) = space {
            if let Some(merc) = piece {
                self.ownership.entry(*sp).and_modify(|x| *x = o);
                merc.owner = o;
                announce_betrayal(&merc, o);
            }
        }
    }

    pub fn move_pieces(
        &mut self,
        o: EntityOwner,
        opponent_b: &mut Bank,
        player_b: &mut Bank,
        mr: &mut MercRegister,
    ) {
        let space_ord: Vec<i32> = match o {
            EntityOwner::Player => (0..=self.max_key).rev().collect(),
            EntityOwner::Opponent => (0..=self.max_key).collect(),
            EntityOwner::None => vec![],
        };

        for (ind, curr) in space_ord.iter().enumerate() {
            if ind != 0 {
                let next = &space_ord[ind - 1];

                let current_merc_op = &mut self.mercs.remove(&curr.clone());
                let next_merc_op = &mut self.mercs.remove(&next.clone());

                if let Some(curr_merc) = current_merc_op.as_mut() {
                    if let (Some(next_merc), true) = (next_merc_op.as_mut(), curr_merc.owner == o) {
                        if next_merc.owner == o {
                            //Swap
                            self.mercs.insert(*curr, *next_merc);
                            self.mercs.insert(*next, *curr_merc);
                        } else {
                            //Battle
                            match handle_invasion(
                                false, curr_merc, next_merc, player_b, opponent_b, mr,
                            ) {
                                InvasionResult::Success => {
                                    self.mercs.insert(*next, *curr_merc);
                                    self.ownership.entry(*next).and_modify(|x| *x = o);
                                }
                                InvasionResult::Stalemate => {
                                    self.mercs.insert(*next, *next_merc);
                                    self.mercs.insert(*curr, *curr_merc);
                                }
                                InvasionResult::RemoveInvader => {
                                    self.mercs.insert(*next, *next_merc);
                                }
                                InvasionResult::RemoveBoth => {}
                            }
                        }
                    } else if curr_merc.owner == o {
                        //Advance
                        self.mercs.insert(*next, *curr_merc);
                        self.ownership.entry(*next).and_modify(|x| *x = o);
                    } else {
                        //Stay Put
                        self.mercs.insert(*curr, *curr_merc);
                        if let Some(nm) = next_merc_op {
                            self.mercs.insert(*next, *nm);
                        }
                    }
                } else if let Some(nm) = next_merc_op {
                    //Put Piece Back
                    self.mercs.insert(*next, *nm);
                }
            }
        }
    }
}
