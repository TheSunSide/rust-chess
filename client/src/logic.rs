#[derive(Debug)]
pub struct ChessBoard {
    board: Board,
    moves: Vec<Move>,
    selected: Option<Square>,
    turn: Color,
    game_over: bool,
    winner: Option<Color>,
    promotion: Option<PieceKind>,
}

#[derive(Debug)]
pub struct ChessPiece {
    piece: Piece,
    square: Square,
}

#[derive(Debug)]
pub struct ChessSquare {
    square: Square,
    piece: Option<Piece>,
    selected: bool,
    legal: bool,
}

#[derive(Debug)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
