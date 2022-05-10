use anime::{error::BrowserError, model::Identifier, Browser};
use clap::Parser;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
    #[error("Invaid query or item not found")]
    NotFoundOrInvalidId,
    #[error("Browser error: {0}")]
    BrowserError(#[from] BrowserError),
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Search {
    #[clap(short, long)]
    /// An anime identifier string that is unique to each series
    pub ident: Option<Identifier<String>>,

    #[clap(short, long)]
    /// A free form string to use for search
    pub search: Option<String>,

    #[clap(short, long)]
    /// Episode number
    pub ep: Option<usize>,
}

async fn run(search: Search) -> Result<(), Error> {
    let mut browser = Browser::new().unwrap();

    if let Some(identifier) = search.ident {
        println!("Searching for {}\n", *identifier);
        println!("{}", browser.get_anime(&identifier).await?);
    } else if let Some(query) = search.search {
        println!("Searching for keywords {}", query);
        let result = browser.search(&query).await?;

        for anime in result {
            println!("{} (ident: {})", anime.1, anime.0);
        }
    } else {
        print!("Please provide either --ident or --search");
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run(Search::parse()).await {
        println!("{}", e);
    }
}
