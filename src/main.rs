#[macro_use]
extern crate diesel;
extern crate dotenv;

mod args;
mod db;
mod models;
mod ops;
mod schema;

use args::EntityType;
use args::RustflixArgs;
use clap::Parser;
use ops::video_ops::handle_video_command;

fn main() {
    let args = RustflixArgs::parse();

    match args.entity_type {
        EntityType::Video(video) => handle_video_command(video),
    };
}
