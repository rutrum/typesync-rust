use api::db;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        help();
        return;
    }

    let command = &args[1];
    match command.as_ref() {
        "scores" => read_scores(&args[2..]),
        _ => help(),
    }
}

fn read_scores(_args: &[String]) {
    let _conn = db::create_connection();
    // can't do stuff with just a SqliteConnection, need DbPool using rocket
}

fn help() {
    println!("  scores <genius_id>: show scores from song with genius_id");
}
