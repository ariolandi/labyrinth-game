use crate::player::{Player, change_coordinates};
use crate::map::{Map, get_player};
use crate::{project_errors::GameError, Position, BATTLE_SCORE, PORTAL_SCORE};

/// Game class - contains the labyrinth's map and players' information
pub struct Game{
    map: Map,
    players: Vec<Player>,
    pub num_of_players: usize,
    pub map_size: usize
}

/// Constructor 
impl Game {
    pub fn new(player_names: Vec<String>, num_of_players: usize, size: usize) -> Game {
        let mut players: Vec<Player> = Vec::new();
        players.push(Player::new(String::new()));
        for (i, name) in player_names.iter().enumerate() {
            players.push(Player::with_code(name.to_string(), i+1));
        }

        Game{
            map: Map::new(size),
            players: players,
            num_of_players: num_of_players + 1,
            map_size: size
        }
    }
}

/// Class methods
impl Game {
    /// Returns String representation of the map
    pub fn display_map(&self) -> String {
        return self.map.display();
    }

    /// Returns string representation of the game info
    pub fn display(&self) -> String {
        let mut result = String::new();
        let mut players = self.players.clone();
        players.remove(0);
        for (i, player) in players.iter().enumerate() {
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

    /// Initialises a portal. Returns false if an Error occured.
    pub fn spawn_portal(&mut self, position: Position) -> bool {
        match self.map.spawn_portal(position) {
            Ok (_) => return true,
            Err(_) => return false,
        }
    }

    /// Initialises a player. Returns false if an Error occured.
    pub fn spawn_player(&mut self, player_code: usize, position: Position) -> bool {
        match self.map.spawn_player(&mut self.players[player_code], position) {
            Ok (_) => return true,
            Err(_) => return false,
        }
    }

    /// Returns a player's name by its code.
    pub fn get_player_name(&self, player_code: usize) -> &str {
        return self.players[player_code].name.as_str();
    }

    /// Simulates a battle between players.
    /// Removes the opponent from the map, adds points to the attaking player and moves it to the new field.
    /// Returns the code of the defeated player.
    pub fn battle(&mut self, player_code: usize, direction: &str) -> usize {   
        let field: Position = change_coordinates(self.players[player_code].coordinates.unwrap(), direction).unwrap();
        let opponent_code: usize = get_player(self.map.get_field(field));
        self.map.kill_player(&mut self.players[opponent_code]);
        self.map.move_player(&mut self.players[player_code], direction).unwrap();
        self.players[player_code].win_points(BATTLE_SCORE);
        return opponent_code;
    }

    /// Simulates walking through a portal.
    /// Adds points to the player.
    pub fn portal(&mut self, player_code: usize) {
        self.players[player_code].win_points(PORTAL_SCORE);
    }

    /// Simulates making a move.
    /// Returns true if the move os successfully done and the game may continue. 
    /// Returns false if the portal was found and the game ends=
    /// Returns an Error otherwise.
    pub fn make_move(&mut self, player_code: usize, direction: &str) -> Result<bool, GameError> {
        match self.map.move_player(&mut self.players[player_code], direction) {
            Ok (_) => return Ok(true),
            Err(GameError::AnotherPlayer) => {
                return Err(GameError::AnotherPlayer);
            },
            Err(GameError::Portal) => {
                return Err(GameError::Portal);
            }
            Err(_) => {
                return Ok(false);
            }
        }
    }

    /// Checks if a player is still in the game.
    pub fn in_game(&self, player_code: usize) -> bool {
        return self.players[player_code].is_alive();
    }

    /// Returns a string representation of the final ranklist.
    pub fn finish(self) -> String {
        let mut ranklist = self.players.clone();
        ranklist.remove(0);
        ranklist = Player::rank(ranklist);
        let mut result = String::new();
        for (i, player) in ranklist.iter().enumerate() {
            result = result + "--- " + &(i+1).to_string() + " ---\n" + &player.display() + "\n";
        }
        return result;
    }
}
