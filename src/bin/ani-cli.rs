use ani_tui::{
    core::{
        cli_args::{Args, Commands},
        dependency_resolution::{Dependency, Resolve},
        Usecase,
    },
    features::watch_anime::domain::usecases::SearchAnime,
};
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Search { title } => {
            let usecase = SearchAnime::new(Dependency::resolve());
            let results = usecase.call(&title).await.unwrap();

            for result in results {
                println!("{}", result.title);
            }
        }
    }
}
