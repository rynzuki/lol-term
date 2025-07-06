mod models;
mod utils {
    pub mod api;
    pub mod config;
    pub mod renderer;
}

use crate::utils::api;
use crate::utils::config;
use crate::utils::renderer;
use clap::{Parser, Subcommand};
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
    Account { account: String },
    Display,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: create cfw to supply riot api data
    dotenv().ok();

    let cli = Cli::parse();
    let config = config::load_config();

    let mut api = api::Api::new(env::var("API_KEY").expect("Missing api key!"));

    match &cli.command {
        Commands::Account { account } => {
            let mut config = config::load_config();

            config.account = Some(account.clone());

            config::save_config(&config);

            Ok(())
        }
        Commands::Display {} => {
            if config.account.is_none() {
                return Err("Please set the account to display first.".into());
            }

            let account = match api.get_account(&config.account.unwrap()).await {
                Ok(account) => account,
                Err(e) => panic!("{}", e),
            };

            let summoner = match api.get_summoner(account.puuid.clone()).await {
                Ok(summoner) => summoner,
                Err(e) => panic!("{}", e),
            };

            let path = api.get_profile_icon(summoner.profile_icon_id).await?;
            renderer::display_summoner_icon(path);
            renderer::display_summoner_stats(&account, &summoner);

            Ok(())
        }
    }
}
