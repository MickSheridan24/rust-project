use crate::mechanics::{board::Board, card::EntityOwner, piece::MercPiece};
use colored::Colorize;
pub fn print_board(board: &mut Board) {
    println!("\n|");
    print!("|");

    for k in 0..=board.max_key {
        let merc = board.mercs.get(&k);
        let own = board.ownership.get(&k).unwrap();
        print_space_data(merc, *own);
    }

    println!("|");
    println!("|\n");
}

pub fn print_space_data(merc_op: Option<&MercPiece>, owner: EntityOwner) {
    print!(
        "{}",
        match owner {
            EntityOwner::Player => "--( ".green(),
            EntityOwner::Opponent => "--( ".red(),
            EntityOwner::None => "--( ".white(),
        }
    );
    if let Some(merc) = merc_op {
        print!(
            " {} ",
            match owner {
                EntityOwner::Player => merc.short_name().green(),
                EntityOwner::Opponent => merc.short_name().red(),
                EntityOwner::None => merc.short_name().white(),
            }
        )
    }
    print!(
        "{}",
        match owner {
            EntityOwner::Player => " )--".green(),
            EntityOwner::Opponent => " )--".red(),
            EntityOwner::None => " )--".white(),
        }
    );
}
