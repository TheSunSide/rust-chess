use serde::Deserialize;

pub type Square = (u8, u8);

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub enum Color {
    White,
    Black,
}

pub type Move = (Square, Square);
pub type Board = Vec<Vec<Option<(PieceKind, Color)>>>;

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct ChessBoard {
    pub board: Board,
    pub moves: Vec<Move>,
    pub selected: Option<Square>,
    pub turn: Color,
    pub game_over: bool,
}

#[allow(dead_code)]
impl ChessBoard {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            moves: vec![],
            selected: None,
            turn: Color::White,
            game_over: false,
        }
    }

    pub fn select(&mut self, square: Square) {
        self.selected = Some(square);
    }
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct ChessPiece {
    pub piece: PieceKind,
    pub square: Square,
}

#[derive(Deserialize, PartialEq, Clone, Debug, Default)]
pub struct ChessSquare {
    pub square: Square,
    pub piece: Option<ChessPiece>,
    pub selected: bool,
    pub legal: bool,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
