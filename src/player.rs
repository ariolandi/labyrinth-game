use crate::project_errors::GameError;
use crate::Position;
use crate::directions;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Player{
    pub name: String,
    score: u32,
    pub player_code: usize,
    pub coordinates: Option<Position>
}

impl Player{
    pub fn new(name: String) -> Player {
        Player{
            name: name,
            player_code: 0,
            score: 0,
            coordinates: None,
        }
    }

    pub fn with_code(name: String, code: usize) -> Player {
        Player{
            name: name,
            player_code: code,
            score: 0,
            coordinates: None,
        }
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.score.partial_cmp(&other.score);
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        return self.score.eq(&other.score);
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.score.cmp(&other.score);
    }
}

impl Eq for Player {}

impl Player{
    pub fn win_points(&mut self, points: u32) {
        self.score = self.score + points;
    }

    pub fn spawn(&mut self, coordinates: Position) {
        self.coordinates = Some(coordinates);
    }

    pub fn set_move(&mut self, direction: &str) {
        self.coordinates = get_coordinates(self.coordinates, direction)
    }

    pub fn display(&self) -> String {
        return self.name.to_string() + "\nscore: " + &self.score.to_string() + "\n";
    }

    pub fn is_alive(&self) -> bool {
        return !(self.coordinates == None) ;
    }

    pub fn rank(players: Vec<Player>) -> Vec<Player> {
        let mut ranklist = players;
        ranklist.sort();
        ranklist.reverse();
        return ranklist;
    }
}

fn get_coordinates(coordinates: Option<Position>, direction: &str) -> Option<Position> {
    match coordinates {
        Some(position) => 
            match change_coordinates(position, direction) {
                Ok(new_coordinates) => Some(new_coordinates),
                Err(_) => None,
            },
        None => None,
    }
}

pub fn change_coordinates(coordinates:Position, direction: &str) -> Result<Position, GameError> {
    let x = coordinates.0;
    let y = coordinates.1;

    if (direction == directions::UP && x == 0) || (direction == directions::LEFT && y == 0) {
        return Err(GameError::Outside);
    }

    if direction == directions::DOWN{
        return Ok(Position(x+1, y));
    }else if direction == directions::UP{
        return Ok(Position(x-1, y));
    }else if direction == directions::LEFT{
        return Ok(Position(x, y-1));
    }else if direction == directions::RIGHT{
        return Ok(Position(x, y+1));
    }else{
        return Err(GameError::InvalidDirection);
    }
}



//---------------------------------------------------------------------------------------------------
// Functions tests

#[test]
fn test_get_coordinates(){
    let position = Position(5, 6);
    assert_eq!(get_coordinates(None, directions::DOWN), None);
    assert_eq!(get_coordinates(Some(position), directions::UP), Some(Position(4, 6)));
    assert_eq!(get_coordinates(Some(position), "jfgs"), None);
    assert_eq!(get_coordinates(Some(Position(0, 0)), directions::UP), None);
}

#[test]
fn test_directions(){
    let position = Position(5, 6);
    assert_eq!(change_coordinates(position, directions::DOWN), Ok(Position(6, 6)));
    assert_eq!(change_coordinates(position, directions::UP), Ok(Position(4, 6)));
    assert_eq!(change_coordinates(position, directions::LEFT), Ok(Position(5, 5)));
    assert_eq!(change_coordinates(position, directions::RIGHT), Ok(Position(5, 7)));
}

#[test]
fn test_directions_error(){
    assert_eq!(change_coordinates(Position(0, 0), "fjhd"), Err(GameError::InvalidDirection));
    assert_eq!(change_coordinates(Position(0, 0), directions::UP), Err(GameError::Outside));

}
