use std::process::exit;

use ani_tui::{
    core::{
        cli_args::{Args, Commands},
        dependency_resolution::{Dependency, Resolve},
        Usecase,
    },
    features::watch_anime::domain::{
        entities::AnimeSearchItem,
        usecases::{GetEpisodesOfAnime, SearchAnime},
    },
};
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Search { title } => {
            let usecase = SearchAnime::new(Dependency::resolve());
            let results = usecase.call(&title).await.unwrap();

            println!();
            for result in results {
                println!(
                    " • {title}\n   {ident}\n",
                    ident = result.ident(),
                    title = result.title
                );
            }
        }
        Commands::ListEps { ident } => {
            let usecase = GetEpisodesOfAnime::new(Dependency::resolve());
            let results = usecase.call(&AnimeSearchItem::new("", &ident)).await;

            if let None = results {
                println!("Error: Nothing found.");
                exit(1);
            }

            let results = results.unwrap();

            println!();
            for result in results {
                println!(
                    " {number:>3} {title}",
                    number = result.ep_number,
                    title = result.title,
                );
            }
            println!()
        }
    }
}
