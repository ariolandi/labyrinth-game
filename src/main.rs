use project::*;
use game::Game;


fn main() {
    let mut game = Game::init();
    game.start();
    while game.next_turn() {}
    Game::end(game);
}