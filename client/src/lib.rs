use dioxus::prelude::*;
use log::info;
use logic::Board;

pub mod logic;
use crate::logic::{ChessBoard, PieceKind, Color};

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
pub(crate) fn chessboard<'a>(cx: Scope<()>, board: &'a mut ChessBoard) -> Element {
    cx.render(rsx! {
        div { class: "board",
            (0..8).map(|i| {
                rsx! {
                    div {
                        (0..8).map(|j| {
                            if (i+j) % 2 == 0 {
                                let class = if board.selected == Some((i,j))  { "square black selected" } else { "square black" };
                                info!("render: class: {}", class);
                                rsx! {
                                    div {   
                                        class: class,
                                        onclick: move |_| {
                                            //info!("Clicked on square {i},{j}");
                                            //board.select((i,j));
                                        }
                                    }
                                }
                            } else {
                                let class = if board.selected == Some((i,j)) { "square selected" } else { "square" };
                                info!("render: class: {}", class);
                                rsx! {
                                    div {   
                                        class: class,
                                        onclick: move |_| {
                                            //info!("Clicked on square {i},{j}");
                                            //board.select((i,j));
                                        } 
                                    }
                                }
                            }

                        })
                    }
                }
            })
        }
    })
}

fn populate_board(board:ChessBoard) -> ChessBoard {
    let mut board = board;
    board.board[0][0] = Some((PieceKind::Rook, Color::White));
    board.board[0][1] = Some((PieceKind::Knight, Color::White));
    board.board[0][2] = Some((PieceKind::Bishop, Color::White));
    board.board[0][3] = Some((PieceKind::Queen, Color::White));
    board.board[0][4] = Some((PieceKind::King, Color::White));
    board.board[0][5] = Some((PieceKind::Bishop, Color::White));
    board.board[0][6] = Some((PieceKind::Knight, Color::White));
    board.board[0][7] = Some((PieceKind::Rook, Color::White));
    for n in 0..8 {
        board.board[1][n] = Some((PieceKind::Pawn, Color::White));
    }

    board.board[7][0] = Some((PieceKind::Rook, Color::Black));
    board.board[7][1] = Some((PieceKind::Knight, Color::Black));
    board.board[7][2] = Some((PieceKind::Bishop, Color::Black));
    board.board[7][3] = Some((PieceKind::Queen, Color::Black));
    board.board[7][4] = Some((PieceKind::King, Color::Black));
    board.board[7][5] = Some((PieceKind::Bishop, Color::Black));
    board.board[7][6] = Some((PieceKind::Knight, Color::Black));
    board.board[7][7] = Some((PieceKind::Rook, Color::Black));
    for n in 0..8 {
        board.board[6][n] = Some((PieceKind::Pawn, Color::Black));
    }
    return board;
}

pub fn app(cx: Scope<()>) -> Element {
    let selected_square = use_state(cx, || (-1,-1));
    let mut matrix: Board = vec![];
    for _ in 0..8 {
        let mut row: Vec<Option<(PieceKind,Color)>> = vec![];
        row.resize(8, None);
        matrix.push(row);
    }

    let mut board = ChessBoard {
        board: matrix,
        moves: vec![],
        selected: None,
        turn: logic::Color::White,
        game_over: false,
    };

    board = populate_board(board);
    
    cx.render(rsx! {
        section { class: "whole",
            style { include_str!("../src/style.css") }
            div {
                h1 { class: "centered", "My application" }
                div { class: "board",
                    (0..8).map(|i| {
                        rsx! {
                            div {
                                (0..8).map(|j| {
                                    if (i+j) % 2 == 0 {
                                        let class = if selected_square.get().0 == i && selected_square.get().1 == j  { "square black selected" } else { "square black" };
                                        info!("render: class: {}", class);
                                        rsx! {
                                            div {   
                                                class: class,
                                                onclick: move |_| {
                                                    //info!("Clicked on square {i},{j}");
                                                    selected_square.set((i,j));
                                                }
                                            }
                                        }
                                    } else {
                                        let class = if selected_square.get().0 == i && selected_square.get().1 == j { "square selected" } else { "square" };
                                        info!("render: class: {}", class);
                                        rsx! {
                                            div {   
                                                class: class,
                                                onclick: move |_| {
                                                    //info!("Clicked on square {i},{j}");
                                                    selected_square.set((i,j));
                                                } 
                                            }
                                        }
                                    }

                                })
                            }
                        }
                    })
                }
            }
        }
        footer { class: "info", p { "A footer" } }
    })
}

#[derive(Props)]
pub struct ExampleProps<'a> {
    set_todos: &'a UseRef<StructExample>,
    id: u32,
}

pub fn todo_entry<'a>(cx: Scope<'a, ExampleProps<'a>>) -> Element {
    render!( li { "allo" } )
}
