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
                                            board.select((i,j));
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
                                            board.select((i,j));
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


fn activate_selected_color(i: u8, j: u8, selected: bool) -> String  {
    let mut class = "square".to_string();
    if (i+j) % 2 == 0 {
        class.push_str(" black");
    }

    if selected {
        class.push_str(" selected");
    }
    return class;
}

#[derive(PartialEq,Props)]
pub struct SquareProps<'a> {
    position: (u8,u8),
    board: &'a mut ChessBoard,
}

#[allow(non_snake_case)]
pub fn Square<'a>(cx: Scope<'a,SquareProps<'a>>) -> Element<'a> {
    let mut class: &UseState<String> = use_state(cx, || String::new());
    let (i,j) = cx.props.position;
    let selected: bool = cx.props.board.selected == Some((i,j));
    class.set(activate_selected_color(i,j,selected));
    cx.render(rsx! {
        div {
            class: "{class}",
            onclick: move |_| {
                info!("Clicked on square {i},{j}");
                cx.props.board.select((i, j));
            }
        }
    })
} 

pub fn app<'a>(cx: Scope<'a,()>) -> Element {
    //let selected_square: &UseState<Option<(usize, usize)>> = use_state(cx, || None);
    let mut matrix: Board = vec![];
    for _ in 0..8 {
        let mut row: Vec<Option<(PieceKind,Color)>> = vec![];
        row.resize(8, None);
        matrix.push(row);
    }

    let mut board:&UseRef<ChessBoard> = use_ref(cx, || (ChessBoard::new(matrix)));
    let read = board.read();
    board.with_mut(|b| populate_board(&mut b.board));
    board.
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
                                    rsx! {
                                        Square {
                                            position: (i,j),
                                            board: (&mut board),
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
