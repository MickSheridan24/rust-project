use crate::mechanics::{
    board::Board,
    card::EntityOwner,
    piece::{merc_piece::MercPiece, struct_piece::StructPiece},
};
use colored::Colorize;

pub fn print_board_construction(board: &Board, owner: EntityOwner) {
    println!("\n|");
    print!("|");

    for k in 0..=board.max_key {
        let structure = board.structures.get(&k);
        let str_own = board.ownership.get(&k).unwrap();
        print_construction(k, structure, *str_own, owner);
    }

    println!("|");
    println!("|\n");
}

fn print_construction(
    i: i32,
    s: Option<&StructPiece>,
    space_owner: EntityOwner,
    struct_owner: EntityOwner,
) {
    if let Some(str) = s {
        if str.is_complete() {
            print!(
                "{}",
                match space_owner {
                    EntityOwner::Player => "--<{ }>--".green(),
                    EntityOwner::Opponent => "--<{ }>--".red(),
                    EntityOwner::None => "--<{ }>--".white(),
                }
            )
        } else {
            let mut disp = "--<".to_owned();
            if space_owner == struct_owner {
                disp.push_str(&i.to_string());
            }
            disp.push_str(">--");

            print!(
                "{}",
                match space_owner {
                    EntityOwner::Player => disp.green(),
                    EntityOwner::Opponent => disp.red(),
                    EntityOwner::None => disp.white(),
                }
            );
        }
    } else {
        let mut disp = "--(".to_owned();
        if space_owner == struct_owner {
            disp.push_str(&i.to_string());
        }
        disp.push_str(")--");
        print!(
            "{}",
            match space_owner {
                EntityOwner::Player => disp.green(),
                EntityOwner::Opponent => disp.red(),
                EntityOwner::None => disp.white(),
            }
        );
    }
}

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
