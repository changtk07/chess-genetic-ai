#[derive(Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Color::White => "white",
            Color::Black => "black",
        };

        write!(f, "{}", s)
    }
}

#[derive(Clone, PartialEq)]
pub struct Piece(pub Color, pub PieceType);

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Piece(Color::White, PieceType::Pawn) => "P",
            Piece(Color::White, PieceType::Rook) => "R",
            Piece(Color::White, PieceType::Knight) => "N",
            Piece(Color::White, PieceType::Bishop) => "B",
            Piece(Color::White, PieceType::Queen) => "Q",
            Piece(Color::White, PieceType::King) => "K",
            Piece(Color::Black, PieceType::Pawn) => "p",
            Piece(Color::Black, PieceType::Rook) => "r",
            Piece(Color::Black, PieceType::Knight) => "n",
            Piece(Color::Black, PieceType::Bishop) => "b",
            Piece(Color::Black, PieceType::Queen) => "q",
            Piece(Color::Black, PieceType::King) => "k",
        };

        write!(f, "{}", s)
    }
}

impl Piece {
    pub fn color(&self) -> &Color {
        &self.0
    }

    pub fn kind(&self) -> &PieceType {
        &self.1
    }
}
