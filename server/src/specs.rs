use crate::data::{Board, ChessBoard, Color, PieceKind};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Deserialize, Serialize)]
pub struct JoinLobby {
    pub id: String,
    pub player: String,
}

#[derive(Deserialize, Serialize)]
pub struct MoveChessPiece {
    pub id: String,
    pub player: String,
    pub from: (u8, u8),
    pub to: (u8, u8),
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, Default)]
pub struct ChessGame {
    pub id: String,
    pub board: ChessBoard,
    pub color_player_1: Color,
    pub player1: String,
    pub player2: String,
    pub is_started: bool,
    pub is_over: bool,
}

pub(crate) struct LobbiesMutex {
    pub(crate) lobbies: Mutex<Vec<ChessGame>>,
}

impl ChessGame {
    pub fn new(player1_color: Color) -> Self {
        Self {
            board: ChessBoard::new(gen_matrix()).clone(),
            id: String::new(),
            player1: String::new().clone(),
            player2: String::new().clone(),
            color_player_1: player1_color,
            is_started: false,
            is_over: false,
        }
    }

    pub fn validify_move(&mut self, from: (u8, u8), to: (u8, u8)) -> bool {
        let (x1, y1) = from;
        let (x2, y2) = to;
        let piece = self.board.board[x1 as usize][y1 as usize].clone();
        if piece.is_none() {
            return false;
        }
        let piece = piece.unwrap();
        if piece.1 != self.board.turn {
            return false;
        }
        if x2 > 7 || y2 > 7 {
            return false;
        }
        let moves = self.board.get_moves(from);
        if !moves.contains(&to) {
            return false;
        }
        true
    }

    /**
     * Moves a piece from one square to another. Only verification is that the piece exists (is not none).
     */
    pub fn move_piece(&mut self, from: (u8, u8), to: (u8, u8)) {
        let (x1, y1) = from;
        let (x2, y2) = to;
        if !self.board.board[from.0 as usize][from.1 as usize].is_none() {
            let piece = self.board.board[x1 as usize][y1 as usize].clone();
            self.board.board[x1 as usize][y1 as usize] = None;
            self.board.board[x2 as usize][y2 as usize] = piece;
        }
    }

    pub fn load_new<'a>(&mut self) {
        self.board.board[0][0] = Some((PieceKind::Rook, Color::White));
        self.board.board[1][0] = Some((PieceKind::Knight, Color::White));
        self.board.board[2][0] = Some((PieceKind::Bishop, Color::White));
        self.board.board[3][0] = Some((PieceKind::Queen, Color::White));
        self.board.board[4][0] = Some((PieceKind::King, Color::White));
        self.board.board[5][0] = Some((PieceKind::Bishop, Color::White));
        self.board.board[6][0] = Some((PieceKind::Knight, Color::White));
        self.board.board[7][0] = Some((PieceKind::Rook, Color::White));
        for n in 0..8 {
            self.board.board[n][1] = Some((PieceKind::Pawn, Color::White));
        }

        self.board.board[0][7] = Some((PieceKind::Rook, Color::Black));
        self.board.board[1][7] = Some((PieceKind::Knight, Color::Black));
        self.board.board[2][7] = Some((PieceKind::Bishop, Color::Black));
        self.board.board[3][7] = Some((PieceKind::Queen, Color::Black));
        self.board.board[4][7] = Some((PieceKind::King, Color::Black));
        self.board.board[5][7] = Some((PieceKind::Bishop, Color::Black));
        self.board.board[6][7] = Some((PieceKind::Knight, Color::Black));
        self.board.board[7][7] = Some((PieceKind::Rook, Color::Black));
        for n in 0..8 {
            self.board.board[n][6] = Some((PieceKind::Pawn, Color::Black));
        }
    }
}

fn gen_matrix() -> Board {
    let mut matrix: Board = vec![];
    for _ in 0..8 {
        let mut row: Vec<Option<(PieceKind, Color)>> = vec![];
        row.resize(8, None);
        matrix.push(row);
    }
    matrix
}
