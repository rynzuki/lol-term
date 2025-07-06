mod models;
mod utils {
    pub mod api;
    pub mod config;
    pub mod renderer;
    pub mod util;
}

use crate::utils::api;
use crate::utils::config;
use crate::utils::renderer;
use crate::utils::util;
use clap::{Parser, Subcommand};
use colored::Colorize;
use dotenv::dotenv;
use std::env;

#[derive(Parser)]
#[command(name = "lolt")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Account { account_name: String },
    Display,
    Matches { amount: Option<u8> },
    Match { match_id: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: create cfw to supply riot api data
    dotenv().ok();

    let cli = Cli::parse();
    let config = config::load_config();

    let mut api = api::Api::new(env::var("API_KEY").expect("Missing api key!"));

    match &cli.command {
        Commands::Account { account_name } => {
            let account = match api.get_account(account_name).await {
                Ok(account) => account,
                Err(_) => {
                    eprintln!("{}", "Account not found!".bright_red());
                    std::process::exit(1);
                }
            };

            let summoner = match api.get_summoner(&account.puuid).await {
                Ok(summoner) => summoner,
                Err(e) => panic!("{}", e),
            };

            let config = config::Config {
                account_name: account_name.clone(),
                puuid: account.puuid,
                summoner_id: summoner.id,
            };
            config::save_config(&config);

            Ok(())
        }
        Commands::Display {} => {
            let summoner = match api.get_summoner(&config.puuid).await {
                Ok(summoner) => summoner,
                Err(e) => panic!("{}", e),
            };

            let path = api.get_profile_icon(summoner.profile_icon_id).await?;
            renderer::display_summoner_icon(path);
            renderer::display_summoner_stats(&config.account_name, &summoner);

            Ok(())
        }
        Commands::Matches { amount } => {
            let amount = match amount {
                Some(amount) => std::cmp::min(*amount, 100),
                None => 10,
            };

            let match_ids = api.get_match_ids(&config.puuid, amount).await?;

            for (i, match_id) in match_ids.iter().enumerate() {
                println!("{} - {}", format!("{}", i + 1).bold(), match_id);
            }

            Ok(())
        }
        Commands::Match { match_id } => {
            let match_data = api.get_match(&match_id).await?;

            let is_win = match util::is_win(&config.summoner_id, &match_data) {
                Ok(win) => win,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };

            println!(
                "{} - {} - {}\n",
                match_id.bold(),
                match is_win {
                    true => "Won".bright_green(),
                    false => "Lost".bright_red(),
                },
                util::format_duration(match_data.info.game_duration)
            );

            Ok(())
        }
    }
}
