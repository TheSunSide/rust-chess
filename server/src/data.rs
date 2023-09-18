// !Important note: This file's structs should be kept in sync with the ones in the client's data.rs file

use serde::{Deserialize, Serialize};

pub type Square = (u8, u8);

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub enum Color {
    #[default]
    White,
    Black,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct ChessGame {
    pub id: String,
    pub board: ChessBoard,
    pub player1: String,
    pub player2: String,
}

#[derive(Deserialize, Serialize)]
pub struct JoinLobby {
    pub id: String,
    pub player: String,
}

pub type Move = (Square, Square);
pub type Board = Vec<Vec<Option<(PieceKind, Color)>>>;

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, Default)]
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

    /** Doesn't check if the color is the right one
     */
    pub fn get_moves(&self, square: Square) -> Vec<Square> {
        let (x, y) = square;
        let piece = self.board[x as usize][y as usize].clone();
        if piece.is_none() {
            return vec![];
        }
        let piece = piece.unwrap();
        let moves: Vec<Square> = match piece.0 {
            PieceKind::Pawn => {
                let mut moves = vec![];
                if piece.1 == Color::White {
                    if x == 1 && self.board[2][y as usize].is_none() {
                        moves.push((x + 2, y));
                    }
                    moves.push((x + 1, y));
                } else {
                    if x == 6 && self.board[5][y as usize].is_none() {
                        moves.push((x - 2, y));
                    }
                    moves.push((x - 1, y));
                }
                moves
            }
            PieceKind::Knight => {
                let mut moves = vec![];
                moves.push((x + 2, y + 1));
                moves.push((x + 2, y - 1));
                moves.push((x - 2, y + 1));
                moves.push((x - 2, y - 1));
                moves.push((x + 1, y + 2));
                moves.push((x + 1, y - 2));
                moves.push((x - 1, y + 2));
                moves.push((x - 1, y - 2));
                moves
            }
            PieceKind::Bishop => self.get_moves_diagonal(square),
            PieceKind::Rook => self.get_moves_line(square),
            PieceKind::Queen => {
                let mut moves = vec![];
                moves.append(&mut self.get_moves_line(square));
                moves.append(&mut self.get_moves_diagonal(square));
                moves
            }
            PieceKind::King => get_king_moves(square),
        };
        moves
    }

    fn get_moves_line(&self, from: Square) -> Vec<Square> {
        let (x, y) = from;
        let mut moves = vec![];
        for i in 1..8 {
            if x + i > 7 {
                break;
            }
            moves.push((x + i, y));
            if self.board[x as usize + i as usize as usize][y as usize].is_some() {
                break;
            }
        }
        for i in 1..8 {
            if x < i {
                break;
            }
            moves.push((x - i, y));
            if self.board[x as usize - i as usize as usize][y as usize].is_some() {
                break;
            }
        }
        for i in 1..8 {
            if y + i > 7 {
                break;
            }
            moves.push((x, y + i));
            if self.board[x as usize][y as usize + i as usize as usize].is_some() {
                break;
            }
        }
        for i in 1..8 {
            if y < i {
                break;
            }
            moves.push((x, y - i));
            if self.board[x as usize][y as usize - i as usize as usize].is_some() {
                break;
            }
        }
        moves
    }
    fn get_moves_diagonal(&self, from: Square) -> Vec<Square> {
        let (x, y) = from;
        let mut moves = vec![];
        for i in 1..8 {
            if x + i > 7 || y + i > 7 {
                break;
            }
            moves.push((x + i, y + i));
            if self.board[x as usize + i as usize as usize][y as usize + i as usize as usize]
                .is_some()
            {
                break;
            }
        }
        for i in 1..8 {
            if x + i > 7 || y < i {
                break;
            }
            moves.push((x + i, y - i));
            if self.board[x as usize + i as usize as usize][y as usize - i as usize as usize]
                .is_some()
            {
                break;
            }
        }
        for i in 1..8 {
            if x < i || y + i > 7 {
                break;
            }
            moves.push((x - i, y + i));
            if self.board[x as usize - i as usize as usize][y as usize + i as usize as usize]
                .is_some()
            {
                break;
            }
        }
        for i in 1..8 {
            if x < i || y < i {
                break;
            }
            moves.push((x - i, y - i));
            if self.board[x as usize - i as usize as usize][y as usize - i as usize as usize]
                .is_some()
            {
                break;
            }
        }
        moves
    }
}

fn get_king_moves(from: Square) -> Vec<Square> {
    let (x, y) = from;
    let mut moves = vec![];
    moves.push((x + 1, y));
    moves.push((x - 1, y));
    moves.push((x, y + 1));
    moves.push((x, y - 1));
    moves.push((x + 1, y + 1));
    moves.push((x + 1, y - 1));
    moves.push((x - 1, y + 1));
    moves.push((x - 1, y - 1));
    moves
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

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
