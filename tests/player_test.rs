use project::player::Player;
use project::Position;

fn player() -> Player {
    return Player::new("test player".to_string());
}

#[test]
fn test_player_display() {
    let expected: String = String::from("test player\nscore: 0\n");
    assert_eq!(player().display(), expected);
}

#[test]
fn test_player_win_points() {
    let mut test_player = player();
    test_player.win_points(100);
    let expected: String = String::from("test player\nscore: 100\n");
    assert_eq!(test_player.display(), expected);
}

#[test]
fn test_player_before_spawn() {
    assert_eq!(player().coordinates, None);
}

#[test]
fn test_spawn() {
    let mut test_player = player();
    test_player.spawn(Position(1, 2));
    assert_eq!(test_player.coordinates, Some(Position(1, 2)));
}

#[test]
fn test_move_player_without_coordinates() {
    let mut test_player = player();
    test_player.set_move("u");
    assert_eq!(test_player.coordinates, None);
}

#[test]
fn test_move_player() {
    let mut test_player = player();
    test_player.spawn(Position(3, 4));
    test_player.set_move("u");
    assert_eq!(test_player.coordinates, Some(Position(2, 4)));
    test_player.set_move("d");
    assert_eq!(test_player.coordinates, Some(Position(3, 4)));
    test_player.set_move("l");
    assert_eq!(test_player.coordinates, Some(Position(3, 3)));
    test_player.set_move("r");
    assert_eq!(test_player.coordinates, Some(Position(3, 4)));
    test_player.set_move("fhgfs");
    assert_eq!(test_player.coordinates, None);
}
