use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AccountDto {
    pub puuid: String,

    #[serde(rename = "gameName")]
    pub game_name: String,

    #[serde(rename = "tagLine")]
    pub tag_line: String,
}

#[derive(Debug, Deserialize)]
pub struct SummonerDto {
    pub puuid: String,

    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i32,

    #[serde(rename = "revisionDate")]
    pub revision_date: i64,

    #[serde(rename = "summonerLevel")]
    pub summoner_level: i64,
}

#[derive(Debug, Deserialize)]
pub struct MatchDto {
    pub metadata: MetaDataDto,
    pub info: InfoDto,
}

#[derive(Debug, Deserialize)]
pub struct MetaDataDto {
    #[serde(rename = "dataVersion")]
    pub data_version: String,

    #[serde(rename = "matchId")]
    pub match_id: String,

    pub participants: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct InfoDto {
    #[serde(rename = "endOfGameResult")]
    pub end_of_game_result: String,

    #[serde(rename = "gameCreation")]
    pub game_creation: i64,

    #[serde(rename = "gameDuration")]
    pub game_duration: i64,

    #[serde(rename = "gameEndTimestamp")]
    pub game_end_timestamp: i64,

    #[serde(rename = "gameId")]
    pub game_id: i64,

    #[serde(rename = "gameMode")]
    pub game_mode: String,

    #[serde(rename = "gameName")]
    pub game_name: String,

    #[serde(rename = "gameStartTimestamp")]
    pub game_start_timestamp: i64,

    #[serde(rename = "gameType")]
    pub game_type: String,

    #[serde(rename = "gameVersion")]
    pub game_version: String,

    #[serde(rename = "mapId")]
    pub map_id: i32,

    #[serde(rename = "participants")]
    pub participants: Vec<ParticipantDto>,

    #[serde(rename = "platformId")]
    pub platform_id: String,

    #[serde(rename = "queueId")]
    pub queue_id: i32,

    #[serde(rename = "teams")]
    pub teams: Vec<TeamDto>,

    #[serde(rename = "tournamentCode")]
    pub tournament_code: String,
}

#[derive(Debug, Deserialize)]
pub struct ParticipantDto {
    pub kills: i32,

    pub assists: i32,

    pub deaths: i32,

    #[serde(rename = "championName")]
    pub champion_name: String,

    #[serde(rename = "summonerId")]
    pub summoner_id: String,

    win: bool,
}

#[derive(Debug, Deserialize)]
pub struct TeamDto {
    pub bans: Vec<BanDto>,

    pub objectives: ObjectivesDto,

    #[serde(rename = "teamId")]
    pub team_id: i32,

    pub win: bool,
}

#[derive(Debug, Deserialize)]
pub struct BanDto {
    #[serde(rename = "championId")]
    pub champion_id: i32,

    #[serde(rename = "pickTurn")]
    pub pick_turn: i32,
}

#[derive(Debug, Deserialize)]
pub struct ObjectivesDto {
    pub baron: ObjectiveDto,

    pub champion: ObjectiveDto,

    pub dragon: ObjectiveDto,

    pub horde: ObjectiveDto,

    pub inhibitor: ObjectiveDto,

    #[serde(rename = "riftHerald")]
    pub rift_herald: ObjectiveDto,

    pub tower: ObjectiveDto,
}

#[derive(Debug, Deserialize)]
pub struct ObjectiveDto {
    first: bool,

    kills: i32,
}
