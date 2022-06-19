use ani_tui::{anime_repo::AnimeRepository, cli_args::*, websites::gogoplay::*};

use clap::Parser;
use std::process::{Command, Stdio};

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let repo = Gogoplay::new();

    match args.command {
        Commands::Search { title } => {
            let results = repo.search(&title).await.unwrap();

            for result in results {
                println!(
                    r#"
 • {title}
   {ident}"#,
                    ident = result.link.as_repr(),
                    title = result.title
                );
            }
        }
        Commands::EpCount { ident } => {
            println!(
                r#""{}" has {} episodes."#,
                repo.detail(EpisodeLink {
                    link: Identifier::from_repr(&ident).unwrap(),
                    title: String::from("")
                })
                .await
                .unwrap()
                .anime_title,
                repo.list_eps(Identifier::from_repr(&ident).unwrap())
                    .await
                    .unwrap()
                    .len(),
            );
        }

        Commands::Detail { ident } => {
            let detail = repo
                .detail(EpisodeLink {
                    link: Identifier::from_repr(&ident).unwrap(),
                    title: String::from(""),
                })
                .await
                .unwrap();
            let ep_count = repo
                .list_eps(Identifier::from_repr(&ident).unwrap())
                .await
                .unwrap()
                .len();

            println!(
                r#"{title}
{eps} episodes, {ident}

{description}"#,
                title = detail.anime_title,
                ident = ident,
                eps = ep_count,
                description = detail.description
            );
        }

        Commands::Watch { ident, ep } => {
            let link = repo
                .watch_link(
                    repo.list_eps(Identifier::from_repr(&ident).unwrap())
                        .await
                        .unwrap()[ep - 1]
                        .clone(),
                )
                .await
                .unwrap();

            println!("Launching MPV");
            Command::new("mpv")
                .arg(link)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
        }
    }
}
