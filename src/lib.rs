pub mod game;
pub mod player;
pub mod map;

pub const MAX_PLAYERS: u32 = 4;
pub const PLAYERCODES: [char; (MAX_PLAYERS+1) as usize] = [' ', '1', '2', '3', '4'];

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Position(pub usize, pub usize);

pub mod directions{
    pub static DOWN: &str = "d";
    pub static UP: &str = "u";
    pub static LEFT: &str = "l";
    pub static RIGHT: &str = "r";
}

pub mod project_errors{
    use std::fmt;
    
    #[derive(Debug, PartialEq)]
    pub enum GameError {
        InvalidCoordinates,
        Outside,
        InvalidField,
        InvalidDirection,
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
                GameError::InvalidCoordinates => "Invalid coordinates!",
                GameError::InvalidDirection => "Invalid direction! Allowed: up, down, left, right",
                GameError::Outside => "Coordinates outside the field",
                GameError::InvalidField => "Invalid field value"
            }
        }
    }

}