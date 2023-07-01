use std::{borrow::BorrowMut, cell::RefCell};

use hashbrown::HashMap;

use crate::mechanics::{
    card::{
        data::card_register::CardRegister,
        EntityOwner::{self, *},
    },
    output::attacks::{announce_attack, announce_betrayal, announce_deployment, announce_result},
    piece::MercPiece,
};

use super::{Board, SpaceData};

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
            Self::deploy_or_oust(RefCell::new(&mut self.space_data), piece, space);
        }
    }

    pub fn deploy_or_oust(
        sdata: RefCell<&mut HashMap<i32, SpaceData>>,
        piece: &mut MercPiece,
        space: i32,
    ) {
        let next_space = match piece.owner {
            EntityOwner::Player => space + 1,
            EntityOwner::Opponent => space - 1,
            EntityOwner::None => panic!("piece has no owner"),
        };

        let mut ex: Option<MercPiece> = Option::None;

        if let Some(data) = sdata.borrow_mut().get_mut(&space) {
            if let Some(existing) = data.get_merc() {
                if existing.owner == piece.owner {
                    ex = Some(*existing);
                    data.set_merc(Option::None);
                    data.set_merc(Some(*piece));
                    data.owner = piece.owner;
                } else {
                    let res = Board::try_attack(piece, existing);
                    announce_result(&res);
                    if res != AttackResult::Victory {
                        println!("Attacker is unable to disengage and is destroyed!");
                    } else {
                        data.set_merc(Option::None);
                        data.set_merc(Some(*piece));
                        data.owner = piece.owner;
                    }
                }
            } else {
                data.set_merc(Some(*piece));
                data.owner = piece.owner;
            }
        }
        if let Some(existing) = ex.as_mut() {
            Board::deploy_or_oust(sdata, existing, next_space);
        }
    }

    pub fn betray_merc(&mut self, r: CardRegister, o: EntityOwner) {
        let found = self.find_merc_space(r);

        if let Some(space) = found {
            space.owner = o;
            space.merc.unwrap().owner = o;
            announce_betrayal(&space.merc.unwrap(), o);
        }
    }

    fn try_attack(attacker: &mut MercPiece, defender: &mut MercPiece) -> AttackResult {
        announce_attack(&attacker, &defender);

        if (attacker.strength >= defender.health) && (defender.strength < attacker.health) {
            AttackResult::Victory
        } else if (attacker.strength >= defender.health) && (defender.strength >= attacker.health) {
            attacker.health -= 1;
            defender.health -= 2;
            AttackResult::Assault
        } else if (attacker.strength < defender.health) && (defender.strength < attacker.health) {
            attacker.health -= 1;
            defender.health -= 1;
            AttackResult::Skirmish
        } else if attacker.morale > 5 {
            AttackResult::FoolsCharge
        } else {
            AttackResult::Panic
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum AttackResult {
    Victory,
    Assault,
    Skirmish,
    Panic,
    FoolsCharge,
}
