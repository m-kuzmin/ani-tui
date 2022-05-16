pub use clap::Parser;

#[derive(Parser)]
#[clap(author, version)]
#[clap(propagate_version = true)]
/// A terminal app to watch anime on GoGoPlay (<https://goload.pro>)
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search for an anime by title
    Search { title: String },
}

/// An anime title + episode number
#[derive(Args)]
pub struct Anime {
    pub title: String,
    pub ep: usize,
}
