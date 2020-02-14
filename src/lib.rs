pub mod game;
pub mod player;
pub mod map;
pub mod utills;
pub mod controllers;
extern crate termion;

pub const MIN_PLAYERS: usize = 2;
pub const MAX_PLAYERS: usize = 4;
pub const PLAYERCODES: [char; (MAX_PLAYERS+1) as usize] = [' ', '1', '2', '3', '4'];
pub const FIELDS: [char; 3] = ['.', '#', '0'];
pub const MAP_SIZE: usize = 10;
pub const BATTLE_SCORE: u32 = 15;
pub const PORTAL_SCORE: u32 = 50;


#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Position(pub usize, pub usize);

pub mod directions{
    pub static DOWN: &str = "s";
    pub static UP: &str = "w";
    pub static LEFT: &str = "a";
    pub static RIGHT: &str = "d";
}

pub mod project_errors{
    use std::fmt;
    
    #[derive(Debug, PartialEq)]
    pub enum GameError {
        InvalidCoordinates,
        Outside,
        InvalidField,
        InvalidDirection,
        AnotherPlayer,
        Portal,
    }
    
    impl fmt::Display for GameError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use std::error::Error;
            write!(f, "{}: {} ", self, self.description())
        }
    }

    impl std::error::Error for GameError {
        fn description(&self) -> &str {
            match  &self {
                GameError::InvalidCoordinates => "Invalid coordinates!\n",
                GameError::InvalidDirection => "Invalid direction! Allowed: up, down, left, right.\n",
                GameError::Outside => "Coordinates outside the field!\n",
                GameError::InvalidField => "Invalid field! You can't reach it.\n",
                GameError::AnotherPlayer => "Battle!",
                GameError::Portal => "Portal has been found!",
            }
        }
    }

}
