#[derive(Clone, PartialEq)]
pub enum Type {
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

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Piece(pub Color, pub Type);

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Piece(Color::White, Type::Pawn) => "P",
            Piece(Color::White, Type::Rook) => "R",
            Piece(Color::White, Type::Knight) => "N",
            Piece(Color::White, Type::Bishop) => "B",
            Piece(Color::White, Type::Queen) => "Q",
            Piece(Color::White, Type::King) => "K",
            Piece(Color::Black, Type::Pawn) => "p",
            Piece(Color::Black, Type::Rook) => "r",
            Piece(Color::Black, Type::Knight) => "n",
            Piece(Color::Black, Type::Bishop) => "b",
            Piece(Color::Black, Type::Queen) => "q",
            Piece(Color::Black, Type::King) => "k",
        };

        write!(f, "{}", s)
    }
}

impl Piece {
    pub fn color(&self) -> &Color {
        &self.0
    }

    pub fn kind(&self) -> &Type {
        &self.1
    }
}
