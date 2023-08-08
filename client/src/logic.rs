use serde::Deserialize;

pub(crate) type Square = (u8, u8);

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub(crate) enum Color {
    White,
    Black,
}

pub(crate) type Move = (Square, Square);
pub(crate) type Board = Vec<Vec<Option<(PieceKind, Color)>>>;

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct ChessBoard {
    pub(crate) board: Board,
    pub(crate) moves: Vec<Move>,
    pub(crate) selected: Option<Square>,
    pub(crate) turn: Color,
    pub(crate) game_over: bool,
}

impl ChessBoard {
    pub(crate) fn select(&mut self, square: Square) {
        self.selected = Some(square);
    }
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct ChessPiece {
    pub(crate) piece: PieceKind,
    pub(crate) square: Square,
}

#[derive(Deserialize, PartialEq, Clone, Debug, Default)]
pub(crate) struct ChessSquare {
    pub(crate) square: Square,
    pub(crate) piece: Option<ChessPiece>,
    pub(crate) selected: bool,
    pub(crate) legal: bool,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub(crate) enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
