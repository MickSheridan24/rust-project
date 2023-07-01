use crate::mechanics::board::AttackResult;
use crate::mechanics::card::EntityOwner;
use crate::mechanics::piece::MercPiece;

use super::prompt_continue;

pub fn announce_attack(agg: &MercPiece, def: &MercPiece) {
    let get_name = |p: &MercPiece| match p.owner {
        EntityOwner::Player => "Player",
        EntityOwner::Opponent => "Opponent",
        _ => "Nobody",
    };

    println!(
        "\n({}) {} attacks ({}) {}!\n",
        get_name(agg),
        agg.name,
        get_name(def),
        def.name
    );
}

pub fn announce_result(r: &AttackResult) {
    match r {
        AttackResult::Victory => println!("\nAn Overwhelming Victory!\n"),
        AttackResult::Assault => println!("\nA Worthy Assault!\n"),
        AttackResult::Skirmish => println!("\nA Mere Skirmish...\n"),
        AttackResult::Panic => println!("\nThe Aggressor Panics!\n"),
        AttackResult::FoolsCharge => println!("\nThe Aggressor Charges into Utter Doom!\n"),
    };
    prompt_continue();
}
