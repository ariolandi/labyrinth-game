use crate::*;
use game::Game;
use crate::{project_errors::GameError, Position};
use utills::*;

impl Game {
    pub fn init() -> Game {
        clear();
        menu();
        clear();
        let (players, number_of_players, map_size) = ask_info();
        return Game::new(players, number_of_players, map_size);
        print!("Debug\n");

    }

    pub fn start(&mut self) {
        for player in 1..self.num_of_players {
            let mut x: usize = rand::random::<usize>() % self.map_size;
            let mut y: usize = rand::random::<usize>()% self.map_size;
            while !self.spawn_player(player, Position(x, y)) {
                x = rand::random::<usize>() % self.map_size;
                y = rand::random::<usize>()% self.map_size;
            }
        }
        let mut x: usize = rand::random::<usize>() % self.map_size;
        let mut y: usize = rand::random::<usize>()% self.map_size;
        while !self.spawn_portal(Position(x, y)) {
            x = rand::random::<usize>() % self.map_size;
            y = rand::random::<usize>()% self.map_size;
        }
        pause();
    }

    pub fn print_map(&self, player_code: usize) {
        clear();
        print_color(self.display_map(), player_code);
    }

    fn play_turn(&mut self, player: usize) -> bool {
        let mut done: bool = false;
        while !done {
            use std::error::Error;
            self.print_map(player);
            print_message("Enter direction: ".to_string());
            flush();
            let direction = &read_str();
            match self.make_move(player, direction) {
                Ok(result) => if result {
                    done = true;
                },
                Err(GameError::AnotherPlayer) => {
                    let dead = self.battle(player, direction);
                    print_message(format!("{} was killed! {} wins {} points!",
                        self.get_player_name(dead), self.get_player_name(player), BATTLE_SCORE));
                    pause();
                    done = true;
                },
                Err(GameError::Portal) => {
                    self.portal(player);
                    print_message(format!("{}! {} wins {} points!", 
                        GameError::Portal.description(), self.get_player_name(player), PORTAL_SCORE));
                    pause();
                    return false;
                },
                Err(e) => {
                    print_error(e.description().to_string());
                    pause();
                },
            }
            flush();
        }
        return true;
    }


    fn play(&mut self, player: usize) -> bool {
        let dice = roll_a_dice();
        print_message(format!("-- PLAYER {} --\n", player));
        print!("Username: {}\n", self.get_player_name(player));
        print!("Moves: {}\n", dice);
        pause();
        for _ in 0..dice {
            if self.play_turn(player) == false {
                return false;
            }
        }
        return true;
    }

    pub fn next_turn(&mut self) -> bool {
        for player_code in 1..self.num_of_players {
            if self.in_game(player_code) {
                if self.play(player_code) == false {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn end(self) {
        clear();
        print_error("GAME FINISHED!\n".to_string());
        print_message(Game::finish(self));
        pause();
    }
}



