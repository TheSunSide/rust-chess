use crate::data::{Board, ChessBoard, Color, PieceKind};
use std::sync::Mutex;

struct ChessGame {
    id: String,
    board: ChessBoard,
    player1: String,
    player2: String,
}

impl ChessGame {
    pub fn new() -> Self {
        Self {
            board: ChessBoard::new(gen_board()).clone(),
            id: String::new(),
            player1: String::new().clone(),
            player2: String::new().clone(),
        }
    }

    pub fn move_piece(&mut self, from: (u8, u8), to: (u8, u8)) {
        let (x1, y1) = from;
        let (x2, y2) = to;
        if !self.board.board[from.0 as usize][from.1 as usize].is_none() {
            let piece = &self.board.board[x1 as usize][y1 as usize];
            self.board.board[x1 as usize][y1 as usize] = None;
            self.board.board[x2 as usize][y2 as usize] = piece.clone();
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

fn gen_board() -> Board {
    let mut matrix: Board = vec![];
    for _ in 0..8 {
        let mut row: Vec<Option<(PieceKind, Color)>> = vec![];
        row.resize(8, None);
        matrix.push(row);
    }
    matrix
}

struct LobbiesMutex {
    lobbies: Mutex<Vec<ChessBoard>>,
}
