use std::borrow::BorrowMut;

use super::{
    card::{data::card_register::CardRegister, structure::Structure, EntityOwner},
    piece::MercPiece,
};
use hashbrown::HashMap;

pub mod merc_control;

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

    fn find_merc_space(&mut self, r: CardRegister) -> Option<&mut SpaceData> {
        for (k, v) in &mut self.space_data {
            if let Some(merc) = v.merc {
                if merc.register == r {
                    return Some(v);
                }
            }
        }
        None
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
