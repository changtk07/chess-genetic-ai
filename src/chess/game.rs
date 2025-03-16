use super::r#move::Move;
use super::state::State;

pub struct Game {
    state: State,
    move_history: Vec<Move>,
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.state)?;
        Ok(())
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: State::new(),
            move_history: Vec::new(),
        }
    }
}
