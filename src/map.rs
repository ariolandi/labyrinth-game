use simple_matrix::Matrix;
use crate::{Position, PLAYERCODES, FIELDS};
use crate::project_errors::GameError;
use crate::player::*;


/// Map class - contains the labyrinth map
/// If a field value is:
///    '.' - it is empty and the players can move on it.
///    '#' - this is a wall.
///    '0' - this is a portal. Walking through it finishes the game.
pub struct Map {
    pub size: usize,
    map: Matrix<char>
}

/// Constructors
impl Map{
    pub fn new(size: usize) -> Map {
        Map{
            size: size,
            map: generate_map(size),
        }
    }

    pub fn from_matrix(size: usize, matrix: &[&[i8]]) -> Map {
        Map{
            size: size,
            map: from_array(matrix),
        }
    }
}


/// Class getters and setters
impl Map{
    fn get(&self, x: usize, y: usize) -> char {
        return *self.map.get(x, y).unwrap();
    }

    pub fn get_field(&self, coordinates: Position) -> char {
        return self.get(coordinates.0, coordinates.1);
    }

    pub fn empty(&self, x: usize, y:usize) -> bool {
        return self.get(x, y) == FIELDS[0];
    }

    fn set_empty(&mut self, coordinates: Position) {
        self.map.set(coordinates.0, coordinates.1, FIELDS[0]);
    }
}

/// Public class methods - controllers for the map 
impl  Map {
    /// Spawns a portal on the map. Returns an Error if the field is not empty.
    pub fn spawn_portal(&mut self, position: Position) -> Result<(), GameError> {
        if self.empty(position.0, position.1) {
            self.map.set(position.0, position.1, FIELDS[2]);
            return Ok(());
        } else {
            return Err(GameError::InvalidCoordinates);
        }
    }

    /// Spawns a player on the map. Returns an Error if the field is not empty.
    pub fn spawn_player(&mut self, player: &mut Player, position: Position) -> Result<(), GameError> {
        if self.empty(position.0, position.1) {
            self.map.set(position.0, position.1, PLAYERCODES[player.player_code]);
            player.spawn(position);
            return Ok(());
        } else {
            return Err(GameError::InvalidCoordinates);
        }
    }

    /// Tries to move a player in a certain direction. 
    /// Returns an Error if the move is invalid, there is another player in the new field (battle occurs)
    /// or there is a portal on the new field (end of the game).
    pub fn move_player(&mut self, player: &mut Player, direction: &str) -> Result<(), GameError> {
        match change_coordinates(player.coordinates.unwrap(), direction) {
            Ok(Position(new_x, new_y)) => {
                if new_x == self.size || new_y == self.size {
                    return Err(GameError::Outside);
                }
                if !self.empty(new_x, new_y) {
                    if self.get(new_x as usize, new_y as usize) == FIELDS[2] {
                        return Err(GameError::Portal);
                    } else if PLAYERCODES.contains(&self.get(new_x as usize, new_y as usize)) {
                        return Err(GameError::AnotherPlayer);
                    } else {
                        return Err(GameError::InvalidCoordinates);
                    }
                }

                let coordinates: Position = player.coordinates.unwrap();
                self.set_empty(coordinates);
                player.set_move(direction);
                self.map.set(new_x, new_y, PLAYERCODES[player.player_code]);
                return Ok(());
            },
            Err(e) => return Err(e),
        }
    }

    /// Emulates killing a player.
    pub fn kill_player(&mut self, player: &mut Player) {
        let coordinates: Position = player.coordinates.unwrap();
        player.coordinates = None;
        self.set_empty(coordinates);
    }

    /// Returns a string representaion of the map.
    pub fn display(&self) -> String {
        let mut result: String = String::new();
        for x in 0..self.size{
            for y in 0..self.size{
                result.push(self.get(x, y));
                result.push(' ');
            }
            result.push('\n');
        }
        return result;
    }
}

fn generate_map(size: usize) -> Matrix<char> {
    let mut map: Matrix<char> = generate_random_map(size);
    while check_if_connected(size, map.clone()) == false {
        map = generate_random_map(size)
    }
    return map;
}

/// Generates a random map.
fn generate_random_map(size: usize) -> Matrix<char> {
    let mut map: Matrix<char> = Matrix::new(size, size);
    for i in 0..size{
        for j in 0..size{
            map.set(i as usize, j as usize, FIELDS[(rand::random::<i8>()%2).abs() as usize]);
        }
    }
    return map;
}

/// Constructs a matrix from an integer 2D array.
fn from_array(matrix: &[&[i8]]) -> Matrix<char> {
    let mut map: Matrix<char> = Matrix::new(matrix.len(), matrix[0].len());
    for (i, row) in matrix.iter().enumerate() {
        for (j, symbol) in row.iter().enumerate() {
            map.set(i, j, FIELDS[*symbol as usize]);
        }
    }
    return map;
} 

fn match_player(field_code: char) -> Result<usize, GameError> {
    if FIELDS.contains(&field_code) || !PLAYERCODES.contains(&field_code){
        return Err(GameError::InvalidField);
    }
    return Ok(field_code.to_string().parse::<usize>().unwrap());
}

pub fn get_player(field: char) -> usize {
    match match_player(field) {
        Ok(player_code) => return player_code,
        Err(_) => return 0,
    }
}

fn check_if_connected(size: usize, matrix: Matrix<char>) -> bool {
    let mut parts = 0;
    let mut map: Matrix<char> = matrix;
    for i in 0..size {
        for j in 0..size {
            if *map.get(i, j).unwrap() == FIELDS[0] {
                traverse(size, &mut map, i, j);
                parts += 1;
            }
        }
    }

    return parts == 1;
}

fn traverse (size: usize, map: &mut Matrix<char>, start_x: usize, start_y: usize) {
    let mut queue: Vec<(usize, usize)> = vec![(start_x, start_y)];
    while !queue.is_empty() {
        let (x, y) = queue.remove(0);
        map.set(x, y, FIELDS[1]);
        if x > 0 && *map.get(x-1, y).unwrap() == FIELDS[0] {
            queue.push((x-1, y));
        }
        if x < size-1 && *map.get(x+1, y).unwrap() == FIELDS[0] {
            queue.push((x+1, y));
        }
        if y > 0 && *map.get(x, y-1).unwrap() == FIELDS[0] {
            queue.push((x, y-1));
        }
        if y < size-1 && *map.get(x, y+1).unwrap() == FIELDS[0] {
            queue.push((x, y+1));
        }
    }
}



// ----------------------------------------------------------------------------------
// Private functions test

#[test]
fn test_from_array() {
    let matrix_array: &[&[i8]] = &[&[0, 1, 1, 0], &[0, 0, 1, 0], &[1, 0, 0, 0], &[1, 1, 0, 0 ]];
    let expected = [['.', '#', '#', '.'], ['.', '.', '#', '.'], ['#', '.', '.', '.'], ['#', '#', '.', '.']];
    let map: Matrix<char> = from_array(matrix_array);
    for i in 0..expected.len() {
        for (j, symbol) in expected[i].iter().enumerate() {
            assert_eq!(symbol, map.get(i, j).unwrap());
        }
    }
}

#[test]
fn test_match_player() {
    assert_eq!(match_player('1'), Ok(1));
    assert_eq!(match_player('2'), Ok(2));
    assert_eq!(match_player('3'), Ok(3));
    assert_eq!(match_player('4'), Ok(4));
}

#[test]
fn test_match_bad_player() {
    assert_eq!(match_player('.'), Err(GameError::InvalidField));
    assert_eq!(match_player('#'), Err(GameError::InvalidField));
    assert_eq!(match_player('0'), Err(GameError::InvalidField));
    assert_eq!(match_player('5'), Err(GameError::InvalidField));
}


#[test]
fn test_get_player() {
    assert_eq!(get_player('0'), 0);
    assert_eq!(get_player('7'), 0);
    assert_eq!(get_player('1'), 1);
    assert_eq!(get_player('2'), 2);
}