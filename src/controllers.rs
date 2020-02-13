use crate::*;
use game::Game;
use crate::{project_errors::GameError, Position};
use utills::*;

impl Game {
    pub fn init() -> Game {
        let (players, number_of_players, map_size) = ask_info();
        return Game::new(players, number_of_players, map_size);
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
    }

    pub fn print_map(&self) {
        print!("{}", self.display_map());
    }

    fn play(&mut self, player: usize) -> bool {
        print!("- PLAYER {} --\n", player);
        let mut done: bool = false;
        while !done {
            use std::error::Error;
            self.print_map();
            print!("Enter direction: ");
            flush();
            let direction = &read_str();
            match self.make_move(player, direction) {
                Ok(result) => if result {
                    done = true;
                },
                Err(GameError::AnotherPlayer) => {
                    let dead = self.battle(player, direction);
                    print!("{} was killed! {} wins {} points!", dead, self.get_player(player), BATTLE_SCORE);
                    done = true;
                },
                Err(GameError::Portal) => {
                    self.portal(player);
                    print!("{}! {} wins {} points!", GameError::Portal.description(), self.get_player(player), PORTAL_SCORE);
                    return false;
                },
                Err(_) => {},
            }
            print!("{}\n", GameError::InvalidDirection.description());
            flush();
        }
        return true;
    }


    pub fn next_turn(&mut self) -> bool {
        return self.play(1);
    }

    pub fn end(self) {
        print!("GAME FINISHED!\n{}", Game::finish(self));
    }
}



