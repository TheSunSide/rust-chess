//! trunk only lets main.rs, not any binary
//!
//!
use chess::*;
use log::LevelFilter;

mod data;

pub fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(Render);
}
