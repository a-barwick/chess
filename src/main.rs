use clap::Parser;
use game::session;

mod game;

#[derive(Parser, Debug)]
#[command(author = "Austin Barwick", version = "0.0.1")]
struct Args {
    #[arg(short = '1', long = "p1")]
    player_one_name: String,

    #[arg(short = '2', long = "p2")]
    player_two_name: String,

    #[arg(short, long = "save")]
    save_game: bool,

    #[arg(long, default_value = "data/saves")]
    save_dir: String,
}

fn main() {
    let args = Args::parse();

    let sess = session::Session::new(
        args.player_one_name,
        args.player_two_name,
        args.save_game,
        args.save_dir,
    );
    if let Err(error) = sess.save() {
        panic!("{}", error.to_string());
    }
}
