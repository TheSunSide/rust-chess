use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

use crate::specs::{JoinLobby, UpdateReq};



mod data;
mod specs;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// !INFO Useful params: https://actix.rs/docs/extractors/
#[post("/new-lobby")]
async fn new_lobby(
    lobbies: web::Data<specs::LobbiesMutex>,
    body: web::Json<String>,
) -> impl Responder {
    let mut lobbies = lobbies.lobbies.lock().unwrap();
    let mut new_lobby = specs::ChessGame::new(data::Color::White);
    new_lobby.load_new();
    new_lobby.player1 = body.to_string();
    let id = uuid::Uuid::new_v4().to_string();
    new_lobby.id = id.clone();
    lobbies.push(new_lobby);
    HttpResponse::Ok().body(format!("New lobby created for {body}! ID: {id}"))
}

#[post("/join-lobby")]
async fn join_lobby(
    lobbies: web::Data<specs::LobbiesMutex>,
    body: web::Json<JoinLobby>,
) -> impl Responder {
    let mut lobbies = lobbies.lobbies.lock().unwrap();
    let mut lobby_iter = lobbies.iter_mut();
    let lobby = lobby_iter.find(|lobby| lobby.id == body.id.to_string());
    if lobby.is_none() {
        return HttpResponse::Ok().body(format!("Lobby {} not found!", body.id));
    } else {
        let unwrapped = lobby.unwrap();
        unwrapped.player2 = body.player.to_string();
        return HttpResponse::Ok().json(unwrapped);
    }
}

#[post("/move")]
async fn move_piece(
    lobbies: web::Data<specs::LobbiesMutex>,
    body: web::Json<specs::MoveChessPiece>,
) -> impl Responder {
    let mut lobbies = lobbies.lobbies.lock().unwrap();
    let mut lobby_iter = lobbies.iter_mut();
    let lobby = lobby_iter.find(|lobby| lobby.id == body.id.to_string());
    if lobby.is_none() {
        return HttpResponse::Ok().body(format!("Lobby {} not found!", body.id));
    } else {
        let lobby = lobby.unwrap();
        if body.player != lobby.player1 && body.player != lobby.player2 {
            return HttpResponse::Ok().body(format!(
                "Player {} not found in lobby {}!",
                body.player, body.id
            ));
        }

        if body.player == lobby.player1 && lobby.color_player_1 != lobby.board.turn {
            return HttpResponse::Ok().body(format!("It's not your turn!"));
        }

        if body.player == lobby.player2 && lobby.color_player_1 == lobby.board.turn {
            return HttpResponse::Ok().body(format!("It's not your turn!"));
        }

        if lobby.validify_move(body.from, body.to) {
            lobby.move_piece(body.from, body.to);
            return HttpResponse::Ok().json(lobby);
        } else {
            return HttpResponse::Ok().body(format!("Invalid move!"));
        }
    }
}

#[get("/lobbies")]
async fn get_lobbies(lobbies: web::Data<specs::LobbiesMutex>) -> impl Responder {
    let lobbies = lobbies.lobbies.lock().unwrap();
    HttpResponse::Ok().json(&*lobbies)
}

#[get("/update")]
async fn get_update(
    lobbies: web::Data<specs::LobbiesMutex>,
    body: web::Json<UpdateReq>
) -> impl Responder {
    let lobbies_vec = lobbies.lobbies.lock().unwrap();
    let lobby = lobbies_vec.iter().find(|lobby| lobby.id == body.id.to_string());
    match lobby {
        Some(lobby) => {
            return HttpResponse::Ok().json(lobby);
        }
        None => {
            return HttpResponse::Ok().body(format!("Lobby {} not found!", body.id));
        }
    }
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn example_get(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let lobbies = web::Data::new(specs::LobbiesMutex {
        lobbies: Mutex::new(vec![]),
    });

    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(counter.clone())
            .app_data(lobbies.clone()) // <- registers the created data
            .route("/", web::get().to(example_get))
            .service(new_lobby)
            .service(get_lobbies)
            .service(join_lobby)
            .service(get_update)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
