use dioxus::prelude::*;
use log::info;
use logic::Board;

pub mod logic;
use crate::logic::{ChessBoard, Color, PieceKind};

#[derive(PartialEq)]
pub enum FilterState {
    All,
    Active,
    Completed,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructExample {
    pub id: u32,
    pub checked: bool,
    pub contents: String,
}

#[inline_props]
#[allow(non_snake_case)]
pub(crate) fn ChessBoardComponent(cx: Scope<()>, board: UseRef<ChessBoard>) -> Element {
    cx.render(rsx! {
        div { class: "board",
            (0..8).map(|i| {
                rsx! {
                    div {
                        (0..8).map(|j| {
                            rsx! {
                                Square {
                                    position: (i,j),
                                    board: board.clone(),
                                }
                            }
                        })
                    }
                }
            })
        }
    })
}

fn populate_board<'a>(board: &'a mut Board) {
    board[0][0] = Some((PieceKind::Rook, Color::White));
    board[0][1] = Some((PieceKind::Knight, Color::White));
    board[0][2] = Some((PieceKind::Bishop, Color::White));
    board[0][3] = Some((PieceKind::Queen, Color::White));
    board[0][4] = Some((PieceKind::King, Color::White));
    board[0][5] = Some((PieceKind::Bishop, Color::White));
    board[0][6] = Some((PieceKind::Knight, Color::White));
    board[0][7] = Some((PieceKind::Rook, Color::White));
    for n in 0..8 {
        board[1][n] = Some((PieceKind::Pawn, Color::White));
    }

    board[7][0] = Some((PieceKind::Rook, Color::Black));
    board[7][1] = Some((PieceKind::Knight, Color::Black));
    board[7][2] = Some((PieceKind::Bishop, Color::Black));
    board[7][3] = Some((PieceKind::Queen, Color::Black));
    board[7][4] = Some((PieceKind::King, Color::Black));
    board[7][5] = Some((PieceKind::Bishop, Color::Black));
    board[7][6] = Some((PieceKind::Knight, Color::Black));
    board[7][7] = Some((PieceKind::Rook, Color::Black));
    for n in 0..8 {
        board[6][n] = Some((PieceKind::Pawn, Color::Black));
    }
}

fn activate_selected_color(i: u8, j: u8, selected: bool) -> String {
    let mut class = "square".to_string();
    if (i + j) % 2 == 0 {
        class.push_str(" black");
    }

    if selected {
        class.push_str(" selected");
    }
    return class;
}

#[inline_props]
#[allow(non_snake_case)]
fn Piece(cx: Scope<()>, board: UseRef<ChessBoard>, position: (u8, u8)) -> Element {
    let piece = use_state(cx, || String::new());
    info!("Rendering Piece...");
    if board.read().board[position.0 as usize][position.1 as usize].is_none() {
        return None;
    }
    let binding = board.read();
    let (kind, color) = binding.board[position.0 as usize][position.1 as usize]
        .as_ref()
        .unwrap();
    piece.set(
        (match kind {
            PieceKind::Pawn => "pawn_",
            PieceKind::Knight => "knight_",
            PieceKind::Bishop => "bishop_",
            PieceKind::Rook => "rook_",
            PieceKind::Queen => "queen_",
            PieceKind::King => "king_",
        })
        .to_string()
            + match color {
                Color::White => "white",
                Color::Black => "black",
            },
    );
    let src = format!("assets/{}.png", piece.get());
    info!("Rendering Piece...");
    cx.render(rsx! {
        img {
            src: "{src}",
            onclick: move |_| {
                info!("Clicked on piece {:?}", piece);
            }
        }
    })
}

#[derive(PartialEq, Props)]
pub struct SquareProps {
    position: (u8, u8),
    board: UseRef<ChessBoard>,
}

#[allow(non_snake_case)]
pub fn Square(cx: Scope<SquareProps>) -> Element {
    let (i, j) = cx.props.position;
    let selected: bool = cx.props.board.read().selected == Some((i, j));
    let class = activate_selected_color(i, j, selected);

    cx.render(rsx! {
        div {
            class: "{class}",
            onclick: move |_| {
                info!("Clicked on square {i},{j}");
                cx.props.board.write().select((i, j));
            }
        }
    })
}

pub fn app<'a>(cx: Scope<'a, ()>) -> Element {
    let mut matrix: Board = vec![];
    for _ in 0..8 {
        let mut row: Vec<Option<(PieceKind, Color)>> = vec![];
        row.resize(8, None);
        matrix.push(row);
    }

    let board: &UseRef<ChessBoard> = use_ref(cx, || {
        let mut board = ChessBoard::new(matrix);
        populate_board(&mut board.board);
        board
    });
    info!("Board: {:?}", board.read().selected);
    cx.render(rsx! {
        section { class: "whole",
            style { include_str!("../src/style.css") }
            div {
                h1 { class: "centered", "My application" }
                ChessBoardComponent { board: board.clone() }
            }
        }
        footer { class: "info", p { "A footer" } }
    })
}
