use simple_matrix::Matrix;
use crate::{Position, MAX_PLAYERS, PLAYERCODES};
use crate::project_errors::GameError;
use crate::player::*;

pub struct Map {
    pub size: usize,
    map: Matrix<i8>
}

impl Map{
    pub fn new(size: usize) -> Map {
        Map{
            size: size,
            map: generate_map(size),
        }
    }

    pub fn from_matrix(size: usize, matrix: Matrix<i8>) -> Map {
        Map{
            size: size,
            map: matrix,
        }
    }
}

impl Map{
    fn get(&self, x: usize, y: usize) -> i8 {
        return *self.map.get(x, y).unwrap();
    }

    pub fn get_field(&self, coordinates: Position) -> i8 {
        return self.get(coordinates.0, coordinates.1);
    }

    pub fn empty(&self, x: usize, y:usize) -> bool {
        return self.get(x, y) == 0;
    }
}

impl  Map {
    pub fn spawn_portal(&mut self, position: Position) -> Result<(), GameError> {
        if self.empty(position.0, position.1) {
            self.map.set(position.0, position.1, 2);
            return Ok(());
        } else {
            return Err(GameError::InvalidCoordinates);
        }
    }

    pub fn spawn_player(&mut self, player: &mut Player, position: Position) -> Result<(), GameError> {
        if self.empty(position.0, position.1) {
            self.map.set(position.0, position.1, -1*player.player_code);
            player.spawn(position);
            return Ok(());
        } else {
            return Err(GameError::InvalidCoordinates);
        }
    }

    pub fn move_player(&mut self, player: &mut Player, direction: &str) -> Result<(), GameError> {
        match change_coordinates(player.coordinates.unwrap(), direction) {
            Ok(Position(new_x, new_y)) => {
                if !self.empty(new_x, new_y) {
                    if self.get(new_x as usize, new_y as usize) == 2{
                        return Err(GameError::Portal);
                    } else if get_player(self.get(new_x as usize, new_y as usize)) > 0 {
                        return Err(GameError::AnotherPlayer);
                    } else {
                        return Err(GameError::InvalidCoordinates);
                    }
                }
                if new_x >= self.size || new_y >= self.size {
                    return Err(GameError::Outside);
                }

                let coordinates: Position = player.coordinates.unwrap();
                self.map.set(coordinates.0, coordinates.1, 0);
                player.set_move(direction);
                self.map.set(new_x, new_y, -1*player.player_code);
                return Ok(());
                },
            Err(e) => return Err(e),
        }
    }

    pub fn kill_player(&mut self, player: &mut Player) {
        let x: usize = player.coordinates.unwrap().0;
        let y: usize = player.coordinates.unwrap().1;
        player.coordinates = None;
        self.map.set(x, y, 0);
    }

    pub fn display(&self) -> String {
        let mut result: String = String::new();
        for x in 0..self.size{
            for y in 0..self.size{
                result.push(get_char(self.get(x, y)).unwrap());
                result.push(' ');
            }
            result.push('\n');
        }
        return result;
    }
}

fn generate_map(size: usize) -> Matrix<i8> {
    let mut map: Matrix<i8> = Matrix::new(size, size);
    for i in 0..size{
        for j in 0..size{
            map.set(i as usize, j as usize, (rand::random::<i8>()%2).abs());
        }
    }
    return map;
}

fn get_char(field: i8) -> Result<char, GameError> {
    if field == 1 {
        return Ok('#');
    } else if field == 0 {
        return Ok('.');
    } else if field == 2 {
        return Ok('0');
    } else {
        return match_player(field as isize); 
    }
}

fn match_player(field: isize) -> Result<char, GameError> {
    if field > 0 {
        return Err(GameError::InvalidField);
    }
    let player_code: usize = (-1*field) as usize;
    if player_code < 1 || player_code > MAX_PLAYERS as usize {
        return Err(GameError::InvalidField);
    } else {
        return Ok(PLAYERCODES[player_code]);
    }
}

pub fn get_player(field: i8) -> usize {
    match match_player(field as isize) {
        Ok(_) => return (-1*field) as usize,
        Err(_) => return 0,
    }
}


//---------------------------------------------------------------------------------------------------
// Private functions test

#[test]
fn test_match_player() {
    assert_eq!(match_player(-1), Ok('1'));
    assert_eq!(match_player(-2), Ok('2'));
    assert_eq!(match_player(-3), Ok('3'));
    assert_eq!(match_player(-4), Ok('4'));
}

#[test]
fn test_match_bad_player() {
    assert_eq!(match_player(0), Err(GameError::InvalidField));
    assert_eq!(match_player(-5), Err(GameError::InvalidField));
    assert_eq!(match_player(1), Err(GameError::InvalidField));
}

#[test]
fn test_get_char() {
    assert_eq!(get_char(0), Ok('.'));
    assert_eq!(get_char(1), Ok('#'));
    assert_eq!(get_char(-1), Ok('1'));
    assert_eq!(get_char(-2), Ok('2'));
    assert_eq!(get_char(-3), Ok('3'));
    assert_eq!(get_char(-4), Ok('4'));
    assert_eq!(get_char(-5), Err(GameError::InvalidField));
}

#[test]
fn test_get_player() {
    assert_eq!(get_player(0), 0);
    assert_eq!(get_player(1), 0);
    assert_eq!(get_player(-1), 1);
    assert_eq!(get_player(-2), 2);
}