use project::{map::Map, player::Player};
use project::project_errors::GameError;
use project::{Position, directions};

fn map() -> Map {
    let matrix_array: &[&[i8]] = &[&[0, 1, 1, 0], &[0, 0, 1, 0], &[1, 0, 0, 0], &[1, 1, 0, 0 ]];
    return Map::from_matrix(4, matrix_array);
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
fn test_map_get() {
    let mut test_map = map();
    assert_eq!(test_map.get_field(Position(0, 0)), '.');
    assert_eq!(test_map.get_field(Position(0, 1)), '#');
    let mut test_player = player();
    match test_map.spawn_player(&mut test_player, Position(0, 0)) {
        Ok(_) => assert_eq!(test_map.get_field(Position(0, 0)), '1'),
        Err(_) => panic!("Something wrong!"),
    }
}

#[test]
fn test_map_spawn_portal() {
    let mut test_map = map();
    let expected: String = "0 # # . \n. . # . \n# . . . \n# # . . \n".to_string();
    match test_map.spawn_portal(Position(0, 0)) {
        Ok(_) => assert_eq!(test_map.display(), expected),
        Err(_) => panic!("Something wrong!"),
    }
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
fn test_move_player_error_invalid_coordinates() {
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
fn test_move_player_error_another_player() {
    let mut test_map = map();
    let mut test_player1 = player();
    let mut test_player2 = player();
    match test_map.spawn_player(&mut test_player1, Position(0, 0)) {
        Ok(_) => {
            test_map.spawn_player(&mut test_player2, Position(1, 0)).ok().unwrap();
            match test_map.move_player(&mut test_player1, directions::DOWN) {
                Ok(_) => panic!("Something wrong!"),
                Err(e) => assert_eq!(e, GameError::AnotherPlayer),
            }
        },
        Err(_) => panic!("Something wrong!"),
    }
}

#[test]
fn test_move_player_error_outside() {
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


#[test]
fn test_kill_player() {
    let mut test_map = map();
    let mut test_player = player();
    test_map.spawn_player(&mut test_player, Position(0, 0)).ok().unwrap();
    test_map.kill_player(&mut test_player);
    let expected: String = ". # # . \n. . # . \n# . . . \n# # . . \n".to_string();
    assert_eq!(map().display(), expected);
}