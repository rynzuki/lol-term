mod models;

use crate::models::{AccountDto, MatchDto, SummonerDto};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use reqwest::Client;
use std::env;
use std::io::Write;
use viuer::{Config, print_from_file};

#[derive(Parser)]
#[command(name = "lolt")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Display { name: String, tag: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: create cfw to supply riot api data
    dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Display { name, tag } => {
            let client = Client::new();

            let account = match get_account(&client, name, tag).await {
                Ok(account) => account,
                Err(e) => panic!("{}", e),
            };

            let summoner = match get_summoner(&client, account.puuid.clone()).await {
                Ok(summoner) => summoner,
                Err(e) => panic!("{}", e),
            };

            let path = get_profile_icon(&client, summoner.profile_icon_id).await?;
            display_summoner_icon(path);
            display_summoner_stats(&account, &summoner);

            let match_ids = get_match_ids(&client, account.puuid.clone(), 5).await?;
            let match_data = get_match(&client, &match_ids[1]).await;

            println!("{:#?}", match_data?);

            Ok(())
        }
    }
}

async fn get_account(
    client: &Client,
    game_name: &str,
    tag_line: &str,
) -> Result<AccountDto, Box<dyn std::error::Error>> {
    let url = format!(
        "https://europe.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}",
        game_name, tag_line
    );

    let res = client
        .get(&url)
        .header(
            "X-Riot-Token",
            env::var("API_KEY").expect("Missing API key"),
        )
        .send()
        .await?;

    if !res.status().is_success() {
        let body = res.text().await?;
        return Err(Box::from(body));
    }

    let account = res.json::<AccountDto>().await?;

    Ok(account)
}

async fn get_summoner(
    client: &Client,
    puuid: String,
) -> Result<SummonerDto, Box<dyn std::error::Error>> {
    let url = format!(
        "https://euw1.api.riotgames.com/lol/summoner/v4/summoners/by-puuid/{}",
        puuid
    );

    let res = client
        .get(&url)
        .header(
            "X-Riot-Token",
            env::var("API_KEY").expect("Missing API key"),
        )
        .send()
        .await?;

    if !res.status().is_success() {
        let body = res.text().await?;
        return Err(Box::from(body));
    }

    let summoner = res.json::<SummonerDto>().await?;

    Ok(summoner)
}

async fn get_match_ids(
    client: &Client,
    puuid: String,
    amount: u8,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://europe.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids?count={}",
        puuid, amount
    );

    let res = client
        .get(&url)
        .header(
            "X-Riot-Token",
            env::var("API_KEY").expect("Missing API key"),
        )
        .send()
        .await?;

    if !res.status().is_success() {
        let body = res.text().await?;
        return Err(Box::from(body));
    }

    let ids = res.json::<Vec<String>>().await?;

    Ok(ids)
}

async fn get_match(
    client: &Client,
    match_id: &String,
) -> Result<MatchDto, Box<dyn std::error::Error>> {
    let url = format!(
        "https://europe.api.riotgames.com/lol/match/v5/matches/{}",
        match_id
    );

    let res = client
        .get(&url)
        .header(
            "X-Riot-Token",
            env::var("API_KEY").expect("Missing API key"),
        )
        .send()
        .await?;

    if !res.status().is_success() {
        let body = res.text().await?;
        return Err(Box::from(body));
    }

    let match_data = res.json::<MatchDto>().await?;

    Ok(match_data)
}

async fn get_profile_icon(
    client: &Client,
    icon_id: i32,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("icons");

    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }

    let path = dir.join(format!("icon_{}.png", icon_id));

    if path.exists() {
        return Ok(path.display().to_string());
    }

    let url = format!(
        "https://ddragon.leagueoflegends.com/cdn/{}/img/profileicon/{}.png",
        env::var("DDRAGON_VERSION").expect("Missing DDragon version key"),
        icon_id
    );

    println!("Downloading {}", url);

    let res = client.get(&url).send().await?;

    if !res.status().is_success() {
        let status_code = res.status();
        let body = res.text().await?;
        return Err(Box::from(format!(
            "Failed to download icon: {}\n{}",
            status_code, body
        )));
    }

    let bytes = res.bytes().await?;

    std::fs::write(&path, &bytes)?;

    Ok(path.display().to_string())
}

fn display_summoner_icon(path: String) {
    let config = Config {
        width: Some(30),
        height: None,
        ..Default::default()
    };

    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().expect("Console flush failed!");

    match print_from_file(path, &config) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    };
}

fn display_summoner_stats(account: &AccountDto, summoner: &SummonerDto) {
    let name = format!("{}#{}", account.game_name, account.tag_line);
    let level = format!("LvL {}", summoner.summoner_level);

    let spaces = " ".repeat(30 - name.len() - level.len());

    println!("{}{}{}", name, spaces, level);
}
