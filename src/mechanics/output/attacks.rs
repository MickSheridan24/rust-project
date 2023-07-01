use crate::mechanics::board::battle_control::AttackResult;
use crate::mechanics::card::EntityOwner;
use crate::mechanics::output::get_owner_name;
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

pub fn announce_betrayal(m: &MercPiece, o: EntityOwner) {
    println!("{} has defected to {}!", m.name, get_owner_name(o))
}

pub fn announce_deployment(m: &MercPiece, o: EntityOwner) {
    println!("{} has deployed {}!", get_owner_name(o), m.name)
}
