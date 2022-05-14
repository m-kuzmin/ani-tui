use anime::{
    core::{cli_args::*, delivery_mechanisms::CachedWebClient, Cache, Usecase},
    features::watch_anime::{
        data::{datasources::GoGoPlayDataSource, repositories::AnimeRepository},
        domain::usecases::SearchAnime,
    },
};

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Search { title } => {
            // TODO get search usecase from DI container
        }
    }
}
