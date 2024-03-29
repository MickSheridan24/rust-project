#[derive(Clone, Copy)]
pub struct Merc {
    pub cost: &'static i32,
    pub betray_cost: &'static i32,
    pub health: &'static i32,
    pub strength: &'static i32,
    pub name: &'static str,
}

pub const merry_landsknechts: Merc = Merc {
    cost: &4,
    betray_cost: &3,
    health: &6,
    strength: &4,
    name: "The Merry Landsknechts",
};
//     BlackRiders,

pub const pavisiers_of_some_renown: Merc = Merc {
    cost: &4,
    betray_cost: &2,
    health: &4,
    strength: &7,
    name: "Pavisiers of Some Renown",
};

pub const sixty_seven_horsemen: Merc = Merc {
    cost: &5,
    betray_cost: &2,
    health: &7,
    strength: &6,
    name: "Sixty-Seven Horsemen",
};

pub const bloody_buccaneers: Merc = Merc {
    cost: &3,
    betray_cost: &1,
    health: &4,
    strength: &4,
    name: "The Bloody Buccaneers",
};

pub const hill_folk: Merc = Merc {
    cost: &2,
    betray_cost: &2,
    health: &3,
    strength: &2,
    name: "Hill Folk",
};

pub const black_riders: Merc = Merc {
    cost: &6,
    betray_cost: &3,
    health: &7,
    strength: &7,
    name: "Black Riders",
};
