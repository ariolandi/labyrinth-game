use crate::player::Player;
use crate::map::Map;


pub struct Game{
    map: Map,
    players: Vec<Player>,
    num_of_players: usize
}
