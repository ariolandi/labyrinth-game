use project::{map::Map, player::Player};
use project::project_errors::GameError;
use project::{Position, directions};
use simple_matrix::Matrix;

fn map() -> Map {
    let matrix_array = [[0, 1, 1, 0], [0, 0, 1, 0], [1, 0, 0, 0], [1, 1, 0, 0 ]];
    let mut matrix: Matrix<i8> = Matrix::new(4, 4);
    for i in 0..4 {
        for j in 0..4 {
            matrix.set(i as usize, j as usize, matrix_array[i][j]);
        }
    }
    return Map::from_matrix(4, matrix);
}

fn player() -> Player {
    return Player::with_code("test player".to_string(), 1);
}


#[test]
fn test_map_display() {
    let expected: String = ". # # . \n. . # . \n# . . . \n# # . . \n".to_string();
    assert_eq!(map().display(), expected);
}

#[test]
fn test_map_spawn_player() {
    let mut test_map = map();
    let mut test_player = player();
    let expected: String = "1 # # . \n. . # . \n# . . . \n# # . . \n".to_string();
    match test_map.spawn_player(&mut test_player, Position(0, 0)) {
        Ok(_) => assert_eq!(test_map.display(), expected),
        Err(_) => panic!("Something wrong!"),
    }
}

#[test]
fn test_map_spawn_player_error() {
    let mut test_map = map();
    let mut test_player = player();
    match test_map.spawn_player(&mut test_player, Position(0, 1)) {
        Ok(_) => panic!("Something wrong!"),
        Err(e) => assert_eq!(e, GameError::InvalidCoordinates),
    }
}

#[test]
fn test_move_player() {
    let mut test_map = map();
    let mut test_player = player();
    let expected: String = ". # # . \n1 . # . \n# . . . \n# # . . \n".to_string();
    match test_map.spawn_player(&mut test_player, Position(0, 0)) {
        Ok(_) => {
            match test_map.move_player(&mut test_player, directions::DOWN) {
                Ok(_) => assert_eq!(test_map.display(), expected),
                Err(_) => panic!("Something wrong!"),
            }
        },
        Err(_) => panic!("Something wrong!"),
    }
}

#[test]
fn test_move_player_error_InvalidCoordinates() {
    let mut test_map = map();
    let mut test_player = player();
    match test_map.spawn_player(&mut test_player, Position(0, 0)) {
        Ok(_) => {
            match test_map.move_player(&mut test_player, directions::RIGHT) {
                Ok(_) => panic!("Something wrong!"),
                Err(e) => assert_eq!(e, GameError::InvalidCoordinates),
            }
        },
        Err(_) => panic!("Something wrong!"),
    }
}

#[test]
fn test_move_player_error_Outside() {
    let mut test_map = map();
    let mut test_player = player();
    match test_map.spawn_player(&mut test_player, Position(0, 0)) {
        Ok(_) => {
            match test_map.move_player(&mut test_player, directions::UP) {
                Ok(_) => panic!("Something wrong!"),
                Err(e) => assert_eq!(e, GameError::Outside),
            }
        },
        Err(_) => panic!("Something wrong!"),
    }
}
