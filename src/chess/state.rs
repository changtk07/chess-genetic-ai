use super::board::*;

pub struct State {
    board: Board,
}

impl State {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
        }
    }

    fn make_move(&mut self, mv: Move) {
        let from = mv.from();
        let to = mv.to();
        let move_type = mv.move_type();
    }
}
