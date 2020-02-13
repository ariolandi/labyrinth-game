use project::player::Player;
use project::Position;

fn player() -> Player {
    return Player::new("test player".to_string());
}

fn player_with_points(points: u32) -> Player {
    let mut player = player();
    player.win_points(points);
    return player;
}

#[test]
fn test_player_display() {
    let expected: String = String::from("test player\nscore: 0\n");
    assert_eq!(player().display(), expected);
}

#[test]
fn test_player_win_points() {
    let test_player = player_with_points(100);
    let expected: String = String::from("test player\nscore: 100\n");
    assert_eq!(test_player.display(), expected);
}

#[test]
fn test_player_before_spawn() {
    assert_eq!(player().coordinates, None);
}

#[test]
fn test_player_is_alive() {
    let mut test_player = player();
    assert_eq!(test_player.is_alive(), false);
    test_player.coordinates = Some(Position(1, 4));
    assert_eq!(test_player.is_alive(), true);
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

#[test]
fn test_player_eq_true() {
    let player1 = player();
    let player2 = player();
    assert_eq!(player1 == player2, true);
}

#[test]
fn test_player_eq_false() {
    let player1 = player();
    let player2 = player_with_points(20);
    assert_eq!(player1==player2, false);
}

#[test]
fn test_player_cmp() {
    let player1 = player_with_points(10);
    let player2 = player_with_points(20);
    assert_eq!(player1 < player2, true);
    assert_eq!(player2 < player1, false);
    assert_eq!(player2 > player1, true);
    assert_eq!(player1 > player2, false);
}

#[test]
fn test_ranking() {
    let sorted: Vec<Player> = Player::rank(vec![player_with_points(20), player(), player_with_points(100)]);
    let expected: Vec<Player> = vec![player_with_points(100), player_with_points(20), player()];
    
    for i in 0..3 {
        assert_eq!(sorted[i]==expected[i], true);
    }
}
