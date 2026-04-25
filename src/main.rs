use chess::State;

mod chess;

fn main() {
    let game = State::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{}", game);
    let moves = game.generate_moves();
    print!("{:?}, ", moves);
}
