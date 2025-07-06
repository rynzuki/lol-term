use crate::models::MatchDto;

pub fn is_win(
    summoner_id: &String,
    match_data: &MatchDto,
) -> Result<bool, Box<dyn std::error::Error>> {
    let participant = match_data
        .info
        .participants
        .iter()
        .find(|&p| p.summoner_id == *summoner_id);

    match participant {
        Some(participant) => Ok(participant.win),
        None => Err("The selected summoner was not in this game.".into()),
    }
}

pub fn format_duration(duration: u32) -> String {
    let minutes = duration / 60;
    let seconds = duration % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
