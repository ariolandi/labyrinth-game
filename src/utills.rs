use crate::*;
use termion::color;



fn greeting_message() {
    print!("Hello, my friend, and wellcome!\n\n");
}

fn allowed_directions() {
    print!("Allowed directions are up ({color}{}{reset}), down ({color}{}{reset}), left ({color}{}{reset}) and right ({color}{}{reset}).\n",
        directions::UP, directions::DOWN, directions::LEFT, directions::RIGHT,
        color = color::Fg(color::LightBlue), reset = color::Fg(color::Reset));
}

fn map_legend() {
    let reset = color::Fg(color::Reset);
    print!("The map contains the following symbols: \n");
    print!("   {color}{}{reset} - an empty field\n", FIELDS[0], color = color::Fg(color::Blue), reset = reset);
    print!("   {color}{}{reset} - a wall\n", FIELDS[1], color = color::Fg(color::LightYellow), reset = reset);
    print!("   {color}{}{reset} - a portal\n", FIELDS[2], color = color::Fg(color::LightCyan), reset = reset);
    print!("   {{{color}number{reset}}} - the current player\n", color = color::Fg(color::LightGreen), reset = reset);
    print!("   {{{color}number{reset}}} - the other players\n", color = color::Fg(color::LightRed), reset = reset);
}

pub fn help() {
    print!("The game can be played by between {} and {} players. ", MIN_PLAYERS, MAX_PLAYERS);
    print!("Their main goal is to exit a labyrinth by walking through a portal.\n");
    print!("In the beginning of the game the players and the portal are randomly located on a map.\n");
    map_legend();
    print!("The players take turns to roll a dice and make moves. ");
    allowed_directions();
    print!("If one player walks on a field of the map with another player on it, a battle begins. ");
    print!("During the battle the attacked player is killed and the attacking player wins {color}{}{reset} points. ",
        BATTLE_SCORE, color = color::Fg(color::LightBlue), reset = color::Fg(color::Reset));
    print!("If a player is dead, he can't make anymore moves, but he participate in the final ranklist with their current points.\n");
    print!("If a player walks on a field with a portal on it, he wins {color}{}{reset} points and the game ends.\n",
        PORTAL_SCORE, color = color::Fg(color::LightBlue), reset = color::Fg(color::Reset));
    pause();
}

pub fn menu() {
    let rules: &str = "r";
    greeting_message();
    print!("If you want to read rules, enter {color}{}{reset}.\n", rules, color = color::Fg(color::LightBlue), reset = color::Fg(color::Reset));
    print!("If you want to start the game, press Enter.\n>>");
    flush();
    let input = read_str();
    if input == rules {
        help();
    }
}



pub fn clear(){
    use std::process::Command;
    let output = Command::new("clear").output().unwrap();
    print!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn read_str() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input.replace("\n", "");
}

pub fn read_num() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input.replace("\n", "").parse::<usize>().unwrap();
}

pub fn flush() {
    use std::io::prelude::*;  
    std::io::stdout().flush().ok();
}

pub fn pause() {
    print!("\nPress Enter to continue...");
    flush();
    read_str();
    clear();
}

pub fn sleep() {
    std::thread::sleep(std::time::Duration::from_millis(300));
}

fn add_player(players: &mut Vec<String>, username: String) -> bool {
    for player in players.iter() {
        if *player == username {
            return false;
        }
    }
    players.push(username);
    return true;
}

pub fn print_error(message: String) {
    print!("{}{}{}", color::Fg(color::LightRed), message, color::Fg(color::Reset));
    flush();
}

pub fn print_message(message: String) {
    print!("{}{}{}", color::Fg(color::LightYellow), message, color::Fg(color::Reset));
    flush();
}

fn read_players(num: usize) -> Vec<String> {
    let mut players: Vec<String> = Vec::new();
    let mut i: usize = 0;
    while i < num {
        print_message(format!("Enter Player {} username: ", i+1));
        let username = read_str();
        if add_player(&mut players, username) {
            i += 1;
        } else {
            print_error("Sorry! This username is already taken. Try again.\n".to_string());
        }
    }
    return players
}

pub fn ask_info() -> (Vec<String>, usize, usize) {
    print_message("Enter number of players: ".to_string());
    let mut number_of_players: usize = read_num();
    while number_of_players > MAX_PLAYERS || number_of_players < MIN_PLAYERS {
        if number_of_players > MAX_PLAYERS {
            print_error(format!("Sorry! Maximum players allowed: {}\n", MAX_PLAYERS));
        } else {
            print_error(format!("Sorry! Minimum players allowed: {}\n", MIN_PLAYERS));
        }
        print_message("Enter number of players: ".to_string());
        number_of_players = read_num();
    }

    let map_size: usize = MAP_SIZE; 

    return (read_players(number_of_players), number_of_players, map_size);
}

fn dice_animation() {
    clear();
    let animation: [&str; 4] = ["\\", "|", "/", "--"];
    for i in 0..4 {
        print!("{}", animation[i]);
        flush();
        sleep();
        clear();
    }
}

pub fn roll_a_dice() -> usize {
    for _ in 0..3 {
        dice_animation();
    }
    return 2 + rand::random::<usize>()%5;
}

pub fn print_color(string: String, player_code: usize) {
    for symbol in string.chars() {
        if symbol == FIELDS[0] {
            print!("{}{}", color::Fg(color::Blue), symbol);
        } else if symbol == FIELDS[2] {
            print!("{}{}", color::Fg(color::LightCyan), symbol);
        } else if PLAYERCODES.contains(&symbol) {
            if symbol == PLAYERCODES[player_code] {
                print!("{}{}", color::Fg(color::LightGreen), symbol);
            } else {
                print!("{}{}", color::Fg(color::LightRed), symbol);
            }
        } else {
            print!("{}{}", color::Fg(color::Reset), symbol);
        }
    }
}