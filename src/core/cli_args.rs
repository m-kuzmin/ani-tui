pub use clap::Parser;

#[derive(Parser)]
#[clap(author, version)]
#[clap(propagate_version = true)]
/// A terminal app to watch anime on GoGoPlay (<https://goload.pro>)
pub struct Args {
    /// A commmand
    #[clap(subcommand)]
    pub command: Commands,
}

/// Supported CLI commnands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum Commands {
    /// Search for an anime by title
    Search {
        /// Anime title
        title: String,
    },
    /// Get a list of episodes for anime identifier.
    ListEps {
        /// An anime identifier
        ident: String,
    },
}

/// An anime title + episode number
#[derive(Args)]
pub struct Anime {
    /// Anme title
    pub anime: String,
    /// Aniem episode
    pub ep: usize,
}
