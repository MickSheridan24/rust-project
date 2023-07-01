use crate::mechanics::{
    board::{Board, SpaceData},
    card::EntityOwner,
};
use colored::Colorize;
pub fn print_board(board: &mut Board) {
    println!("\n|");
    print!("|");

    for k in 0..(board.max_key + 1) {
        let data = board.space_data.get_mut(&k);
        match data {
            Some(d) => {
                print_space_data(d);
            }
            None => print!("--(  )--"),
        }
    }

    println!("|");
    println!("|\n");
}

pub fn print_space_data(data: &mut SpaceData) {
    let owner = data.owner;
    print!(
        "{}",
        match owner {
            EntityOwner::Player => "--( ".green(),
            EntityOwner::Opponent => "--( ".red(),
            EntityOwner::None => "--( ".black(),
        }
    );
    if let Some(merc) = data.get_merc() {
        print!(
            " {} ",
            match owner {
                EntityOwner::Player => merc.short_name().green(),
                EntityOwner::Opponent => merc.short_name().red(),
                EntityOwner::None => merc.short_name().black(),
            }
        )
    }
    print!(
        "{}",
        match owner {
            EntityOwner::Player => " )--".green(),
            EntityOwner::Opponent => " )--".red(),
            EntityOwner::None => " )--".black(),
        }
    );
}
