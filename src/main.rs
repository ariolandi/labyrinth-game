use project::*;
use map::Map;
use player::Player;
use Position;
use std::error::Error;

fn main() {
    let mut test_map = Map::new(5);
    let mut test_player = Player::with_code("test".to_string(), 1);
    match test_map.spawn_player(&mut test_player, Position(0, 0)) {
        Ok(_) => {
            match test_map.move_player(&mut test_player, "up") {
                Ok(_) => print!("{}", test_map.display()),
                Err(e) => print!("{}", e.description()),
            }
        },
        Err(_) => print!("fjfhs"),
    }

}