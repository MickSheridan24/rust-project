use super::{
    card::{merc::Merc, structure::Structure, EntityOwner},
    output::attacks::{announce_attack, announce_result},
    piece::MercPiece,
};
use bevy::reflect::erased_serde::__private::serde::__private::de;
use hashbrown::HashMap;
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
};

pub struct Board {
    pub max_key: i32,
    pub connections: HashMap<i32, Vec<SpaceConnection>>,
    pub space_data: HashMap<i32, SpaceData>,

    pub player_start: i32,
    pub opponent_start: i32,
}

impl Board {
    pub fn init_straight() -> Board {
        let mut board = Board {
            connections: HashMap::new(),
            space_data: HashMap::new(),
            player_start: 0,
            opponent_start: 5,
            max_key: 5,
        };

        board.insert_connected(0, 1);
        board.insert_connected(1, 2);
        board.insert_connected(2, 3);
        board.insert_connected(3, 4);
        board.insert_connected(4, 5);

        for k in 0..6 {
            board.space_data.insert(k, SpaceData::init());
        }

        board
    }
    pub fn deploy_merc(&mut self, merc: Merc, owner: EntityOwner) {
        let space = match owner {
            EntityOwner::Player => self.player_start,
            EntityOwner::Opponent => self.opponent_start,
            EntityOwner::None => -1,
        };
        let mut piece = MercPiece::from_card(merc, owner, space);
        Self::deploy_or_oust(RefCell::new(&mut self.space_data), &mut piece, space);
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

        let mut ex: Option<MercPiece> = None;

        if let Some(data) = sdata.borrow_mut().get_mut(&space) {
            if let Some(existing) = data.get_merc() {
                if existing.owner == piece.owner {
                    ex = Some(*existing);
                    data.set_merc(None);
                    data.set_merc(Some(*piece));
                    data.owner = piece.owner;
                } else {
                    let res = Board::try_attack(piece, existing);
                    announce_result(&res);
                    if res != AttackResult::Victory {
                        println!("Attacker is unable to disengage and is destroyed!");
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

    fn insert_connected(&mut self, f: i32, to: i32) {
        if (f > self.max_key) || (to > self.max_key) {
            panic!("board option inserted greater than given max key")
        }
        self.connections
            .entry(f)
            .and_modify(|v| v.push(SpaceConnection::create(f, to)))
            .or_insert(vec![SpaceConnection::create(f, to)]);

        self.connections
            .entry(to)
            .and_modify(|v| v.push(SpaceConnection::create(to, f)))
            .or_insert(vec![SpaceConnection::create(to, f)]);
    }
}

pub struct SpaceData {
    pub structure: Option<Structure>,
    merc: Option<MercPiece>,
    pub owner: EntityOwner,
}

impl SpaceData {
    pub fn init() -> SpaceData {
        SpaceData {
            structure: None,
            merc: None,
            owner: EntityOwner::None,
        }
    }

    pub fn get_merc(&mut self) -> Option<&mut MercPiece> {
        match self.merc.as_mut() {
            Some(m) => Some(m),
            None => None,
        }
    }
    pub fn set_merc(&mut self, m: Option<MercPiece>) {
        if let Some(mer) = m {
            if let Some(_) = self.merc {
                panic!("tried to oust existing merc");
            }
            self.owner = mer.owner;
        }

        self.merc = m;
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

pub struct SpaceConnection {
    pub from: i32,
    pub to: i32,
    pub connected: bool,
    pub terrain: Terrain,
    pub connection_type: ConnectionType,
}
impl SpaceConnection {
    fn create(from: i32, to: i32) -> SpaceConnection {
        SpaceConnection {
            from,
            to,
            connected: true,
            terrain: Terrain::None,
            connection_type: ConnectionType::Path,
        }
    }
}

pub enum Terrain {
    None,
    Forest,
    Mountain,
    River,
    Sea,
}

pub enum ConnectionType {
    None,
    Path,
    Road,
}
