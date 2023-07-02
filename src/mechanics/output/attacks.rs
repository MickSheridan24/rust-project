use crate::mechanics::board::battle_control::AttackResult;
use crate::mechanics::card::data::card_register::CardRegister;
use crate::mechanics::card::EntityOwner;
use crate::mechanics::output::get_owner_name;
use crate::mechanics::piece::merc_piece::MercPiece;

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
        AttackResult::Victory(_) => println!("\nAn Overwhelming Victory!\n"),
        AttackResult::Assault(_) => println!("\nA Worthy Assault!\n"),
        AttackResult::Skirmish(_) => println!("\nA Mere Skirmish...\n"),
        AttackResult::Panic => println!("\nThe Aggressor Panics!\n"),
        AttackResult::FoolsCharge(_) => println!("\nThe Aggressor Charges into Utter Doom!\n"),
    };
    prompt_continue();
}

pub fn announce_betrayal(m: &MercPiece, o: EntityOwner) {
    println!("{} has defected to {}!", m.name, get_owner_name(o))
}

pub fn announce_deployment(m: &MercPiece, o: EntityOwner) {
    println!("{} has deployed {}!", get_owner_name(o), m.name)
}

pub fn announce_destruction(r: &CardRegister) {
    println!("{} has been defeated!", r.get_card().get_name());
}
