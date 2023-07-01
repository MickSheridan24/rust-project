use std::{borrow::BorrowMut, cell::RefCell};

use hashbrown::HashMap;

use crate::mechanics::{
    card::{
        data::card_register::CardRegister,
        EntityOwner::{self, *},
    },
    output::attacks::{announce_betrayal, announce_deployment, announce_result},
    piece::MercPiece,
};

use super::{
    battle_control::{try_attack, AttackResult},
    Board,
};

impl Board {
    pub fn deploy_merc(&mut self, r: CardRegister, owner: EntityOwner) {
        let space = match owner {
            Player => self.player_start,
            Opponent => self.opponent_start,
            None => -1,
        };
        let mut piece_res = MercPiece::from_card(r, owner, space);
        if let Some(piece) = piece_res.borrow_mut() {
            announce_deployment(&piece, owner);
            self.deploy_or_oust(piece, space);
        }
    }

    pub fn deploy_or_oust(&mut self, piece: &mut MercPiece, space: i32) {
        let next_space = match piece.owner {
            EntityOwner::Player => space + 1,
            EntityOwner::Opponent => space - 1,
            EntityOwner::None => panic!("piece has no owner"),
        };

        let mut ex: Option<MercPiece> = Option::None;

        if let Some(existing) = self.mercs.get_mut(&space) {
            if existing.owner == piece.owner {
                ex = Some(*existing);
                self.mercs.entry(space).and_modify(|x| *x = *piece);
            } else {
                let res = try_attack(piece, existing);
                announce_result(&res);
                if res != AttackResult::Victory {
                    println!("Attacker is unable to disengage and is destroyed!");
                } else {
                    self.mercs
                        .entry(space)
                        .and_modify(|x| *x = *piece)
                        .or_insert(*piece);
                    self.ownership.entry(space).and_modify(|x| *x = piece.owner);
                }
            }
        } else {
            self.mercs
                .entry(space)
                .and_modify(|x| *x = *piece)
                .or_insert(*piece);
            self.ownership.entry(space).and_modify(|x| *x = piece.owner);
        }
        if let Some(existing) = ex.as_mut() {
            self.deploy_or_oust(existing, next_space);
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

    pub fn move_pieces(&mut self, o: EntityOwner) {
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
                    if curr_merc.owner == o {
                        let mut dest_merc = *curr_merc;

                        if let Some(next_merc) = next_merc_op.as_mut() {
                            if next_merc.owner == o {
                                self.mercs.insert(*curr, *next_merc);
                            } else {
                                let res = try_attack(curr_merc, next_merc);
                                announce_result(&res);
                                match res {
                                    AttackResult::Victory => (),
                                    AttackResult::FoolsCharge => {
                                        dest_merc = *next_merc;
                                    }
                                    _ => {
                                        dest_merc = *next_merc;
                                        self.mercs.insert(*curr, *curr_merc);
                                    }
                                }
                            }
                        }
                        println!("DEST MERC {}", dest_merc.name);
                        self.ownership
                            .entry(*next)
                            .and_modify(|x| *x = dest_merc.owner);
                        self.mercs.insert(*next, dest_merc);
                    } else {
                        println!("WHOOPS {}, {}", curr_merc.name, curr);
                        self.mercs.insert(*curr, *curr_merc);
                        if let Some(nm) = next_merc_op {
                            println!("Wow, clumsy me {}, {}", nm.name, next);
                            self.mercs.insert(*next, *nm);
                        }
                    }
                } else if let Some(nm) = next_merc_op {
                    println!("Wow, very clumsy me {}, {}", nm.name, next);
                    self.mercs.insert(*next, *nm);
                }
            }
        }
    }
}
