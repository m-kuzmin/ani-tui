use std::process::exit;

use ani_tui::{
    core::{
        cli_args::{Args, Commands},
        dependency_resolution::{Dependency, Resolve},
        Usecase,
    },
    features::watch_anime::domain::{
        entities::AnimeSearchItem,
        usecases::{GetAnimeDetails, GetEpisodesOfAnime, SearchAnime},
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

            if results.is_none() {
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

        Commands::Detail { ident } => {
            let usecase = GetAnimeDetails::new(Dependency::resolve());
            let result = usecase.call(&AnimeSearchItem::new("", &ident)).await;

            if result.is_none() {
                println!("Error: Nothing found.");
                exit(1);
            }
            let result = result.unwrap();

            println!(
                "\n {title}\n [{ident}]\n\n{desc}",
                ident = result.ident(),
                title = result.title,
                desc = result.desc,
            );

            println!();
            for ep in result.eps {
                println!(
                    " {number:>3} {title}",
                    number = ep.ep_number,
                    title = ep.title,
                );
            }
            println!()
        }
    }
}
