use crate::models::{AccountDto, MatchDto, SummonerDto};
use reqwest::Client;
use std::env;

pub struct Api {
    api_key: String,
    client: Client,
}

impl Api {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn get_account(
        &mut self,
        account: &String,
    ) -> Result<AccountDto, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = account.split("#").collect();

        if parts.len() != 2 {
            return Err("Invalid account name".into());
        }

        let url = format!(
            "https://europe.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}",
            parts[0], parts[1],
        );

        let res = self
            .client
            .get(&url)
            .header("X-Riot-Token", &self.api_key)
            .send()
            .await?;

        if !res.status().is_success() {
            let body = res.text().await?;
            return Err(Box::from(body));
        }

        let account = res.json::<AccountDto>().await?;

        Ok(account)
    }

    pub async fn get_summoner(
        &mut self,
        puuid: &String,
    ) -> Result<SummonerDto, Box<dyn std::error::Error>> {
        let url = format!(
            "https://euw1.api.riotgames.com/lol/summoner/v4/summoners/by-puuid/{}",
            puuid
        );

        let res = self
            .client
            .get(&url)
            .header("X-Riot-Token", &self.api_key)
            .send()
            .await?;

        if !res.status().is_success() {
            let body = res.text().await?;
            return Err(Box::from(body));
        }

        let summoner = res.json::<SummonerDto>().await?;

        Ok(summoner)
    }

    pub async fn get_match_ids(
        &mut self,
        puuid: &String,
        amount: u8,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://europe.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids?count={}",
            puuid, amount
        );

        let res = self
            .client
            .get(&url)
            .header("X-Riot-Token", &self.api_key)
            .send()
            .await?;

        if !res.status().is_success() {
            let body = res.text().await?;
            return Err(Box::from(body));
        }

        let ids = res.json::<Vec<String>>().await?;

        Ok(ids)
    }

    pub async fn get_match(
        &mut self,
        match_id: &String,
    ) -> Result<MatchDto, Box<dyn std::error::Error>> {
        let url = format!(
            "https://europe.api.riotgames.com/lol/match/v5/matches/{}",
            match_id
        );

        let res = self
            .client
            .get(&url)
            .header("X-Riot-Token", &self.api_key)
            .send()
            .await?;

        if !res.status().is_success() {
            let body = res.text().await?;
            return Err(Box::from(body));
        }

        let match_data = res.json::<MatchDto>().await?;

        Ok(match_data)
    }

    pub async fn get_profile_icon(
        &mut self,
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

        let res = self.client.get(&url).send().await?;

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
}
