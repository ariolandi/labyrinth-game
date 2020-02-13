use crate::*;

pub fn clear(){
    use std::process::Command;
    let output = Command::new("clear").output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
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
    std::io::stdout().flush().ok().expect("Could not flush stdout");
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

fn read_players(num: usize) -> Vec<String> {
    let mut players: Vec<String> = Vec::new();
    let mut i: usize = 0;
    while i < num {
        print!("Enter PLayer {} username: ", i+1);
        flush();
        let username = read_str();
        if add_player(&mut players, username) {
            i += 1;
        } else {
            print!("Sorry! This username is already taken. Try again.\n");
            flush();
        }
    }
    return players
}

pub fn ask_info() -> (Vec<String>, usize, usize) {
    print!("Enter number of players: ");
    flush();
    let mut number_of_players: usize = read_num();
    while number_of_players > MAX_PLAYERS || number_of_players < MIN_PLAYERS {
        if number_of_players > MAX_PLAYERS {
            print!("Sorry! Maximum players allowed: {}\n", MAX_PLAYERS);
        } else {
            print!("Sorry! Minimum players allowed: {}\n", MIN_PLAYERS);
        }
        print!("Enter number of players: ");
        flush();
        number_of_players = read_num();
    }

    let map_size: usize = MAP_SIZE; 

    return (read_players(number_of_players), number_of_players, map_size);
}