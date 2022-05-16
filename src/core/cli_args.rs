pub use clap::Parser;

#[derive(Parser)]
#[clap(author, version)]
#[clap(propagate_version = true)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search for an anime by title
    Search { title: String },
}

#[derive(Args)]
pub struct Anime {
    pub title: String,
    pub ep: usize,
}
