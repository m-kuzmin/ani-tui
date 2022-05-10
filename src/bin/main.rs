use anime::{error::BrowserError, model::Identifier, Browser};
use clap::Parser;

#[derive(Debug)]
pub enum Error {
    NotFoundOrInvalidId,
    BrowserError(BrowserError),
}

impl From<BrowserError> for Error {
    fn from(source: BrowserError) -> Self {
        Self::BrowserError(source)
    }
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let search = Search::parse();

    let mut browser = Browser::new().unwrap();

    if let Some(identifier) = search.ident {
        println!("{}", browser.get_anime(&identifier).await?);
    } else if let Some(query) = search.search {
        let result = browser.search(&query).await?;

        for anime in result {
            println!("{} (Id: {})", anime.1, anime.0);
        }
    } else {
        print!("Please provide either --ident or --search");
    }
    Ok(())
}
