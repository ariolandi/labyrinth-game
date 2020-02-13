use project::*;
use game::Game;


fn main() {
    let mut game = Game::init();
    game.start();
    while game.next_turn() {}
    Game::end(game);

    // let mut a = String::new();
    // let n:usize = stdin().read_line(&mut a).unwrap();
    // print!("{}", a.to_string());
}