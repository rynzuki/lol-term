use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Account {
    pub puuid: String,

    #[serde(rename = "gameName")]
    pub game_name: String,

    #[serde(rename = "tagLine")]
    pub tag_line: String,
}

#[derive(Debug, Deserialize)]
pub struct Summoner {
    pub puuid: String,

    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i32,

    #[serde(rename = "revisionDate")]
    pub revision_date: i64,

    #[serde(rename = "summonerLevel")]
    pub summoner_level: i64,
}

#[derive(Debug, Deserialize)]
pub struct Match {
    pub metadata: MetaData,
    pub info: Info,
}

#[derive(Debug, Deserialize)]
pub struct MetaData {
    #[serde(rename = "dataVersion")]
    pub data_version: String,

    #[serde(rename = "matchId")]
    pub match_id: String,

    pub participants: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Info {}
