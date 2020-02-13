use crate::player::{Player, change_coordinates};
use crate::map::Map;
use crate::{project_errors::GameError, Position, BATTLE_SCORE, PORTAL_SCORE};


pub struct Game{
    map: Map,
    players: Vec<Player>,
    pub num_of_players: usize,
    pub map_size: usize
}

impl Game {
    pub fn new(player_names: Vec<String>, num_of_players: usize, size: usize) -> Game {
        let mut players: Vec<Player> = Vec::new();
        for (i, name) in player_names.iter().enumerate() {
            let code: i8 = (i+1) as i8; 
            players.push(Player::with_code(name.to_string(), code));
        }

        Game{
            map: Map::new(size),
            players: players,
            num_of_players: num_of_players + 1,
            map_size: size
        }
    }
}

impl Game {
    pub fn display_map(&self) -> String {
        return self.map.display();
    }

    pub fn display(&self) -> String {
        let mut result = String::new();
        for (i, player) in self.players.iter().enumerate() {
            result.push_str("Player ");
            result.push_str(&i.to_string());
            result.push('\n');
            result.push_str(&player.display());
            result.push('\n');
        }
        result.push_str(&self.map.display());
        result.push('\n');
        return result;
    }

    pub fn spawn_portal(&mut self, position: Position) -> bool {
        match self.map.spawn_portal(position) {
            Ok (_) => return true,
            Err(_) => return false,
        }
    }

    pub fn spawn_player(&mut self, player_code: usize, position: Position) -> bool {
        match self.map.spawn_player(&mut self.players[player_code - 1], position) {
            Ok (_) => return true,
            Err(_) => return false,
        }
    }

    pub fn get_player(&self, player_code: usize) -> &str {
        return self.players[player_code - 1].name.as_str();
    }

    pub fn battle(&mut self, player_code: usize, direction: &str) -> usize {   
        let field: Position = change_coordinates(self.players[player_code - 1].coordinates.unwrap(), direction).unwrap();
        let opponent_code: usize = (-1*self.map.get_field(field)) as usize;
        self.map.kill_player(&mut self.players[opponent_code - 1]);
        self.map.move_player(&mut self.players[player_code - 1], direction).unwrap();
        self.players[player_code - 1].win_points(BATTLE_SCORE);
        return opponent_code;
    }

    pub fn portal(&mut self, player_code: usize) {
        self.players[player_code - 1].win_points(PORTAL_SCORE);
    }

    pub fn make_move(&mut self, player_code: usize, direction: &str) -> Result<bool, GameError> {
        match self.map.move_player(&mut self.players[player_code - 1], direction) {
            Ok (_) => return Ok(true),
            Err(GameError::AnotherPlayer) => {
                return Err(GameError::AnotherPlayer);
            },
            Err(GameError::Portal) => {
                return Err(GameError::Portal);
            }
            Err(e) => {
                use std::error::Error;
                print!("{}", e.description());
                return Ok(false);
            }
        }
    }

    pub fn in_game(&self, player_code: usize) -> bool {
        return self.players[player_code - 1].is_alive();
    }

    pub fn finish(self) -> String {
        let mut ranklist = self.players;
        ranklist = Player::rank(ranklist);
        let mut result = String::new();
        for (i, player) in ranklist.iter().enumerate() {
            result = result + "--- " + &(i+1).to_string() + " ---\n" + &player.display() + "\n";
        }
        return result;
    }
}
