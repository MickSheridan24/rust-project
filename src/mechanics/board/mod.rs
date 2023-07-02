use super::{
    card::{structure::Structure, EntityOwner},
    piece::{merc_piece::MercPiece, struct_piece::StructPiece},
};
use hashbrown::HashMap;

pub mod battle_control;
pub mod merc_control;
pub mod structure_control;

pub struct Board {
    pub max_key: i32,
    pub connections: HashMap<i32, Vec<SpaceConnection>>,
    pub mercs: HashMap<i32, MercPiece>,
    pub structures: HashMap<i32, StructPiece>,
    pub ownership: HashMap<i32, EntityOwner>,

    pub player_start: i32,
    pub opponent_start: i32,
}

impl Board {
    pub fn init_straight() -> Board {
        let mut board = Board {
            connections: HashMap::new(),
            player_start: 0,
            opponent_start: 5,
            max_key: 5,
            mercs: HashMap::new(),
            structures: HashMap::new(),
            ownership: HashMap::new(),
        };

        board.insert_connected(0, 1);
        board.insert_connected(1, 2);
        board.insert_connected(2, 3);
        board.insert_connected(3, 4);
        board.insert_connected(4, 5);

        board.ownership.insert(0, EntityOwner::Player);
        board.ownership.insert(5, EntityOwner::Opponent);
        for k in 1..5 {
            board.ownership.insert(k, EntityOwner::None);
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
