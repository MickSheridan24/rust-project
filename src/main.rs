mod mechanics;
use mechanics::game::*;
use mechanics::mocks::*;
fn main() {
    println!("Hello, game!");

    let mut game = Game::init(player_deck(), opponent_deck());
    game.run();
}
