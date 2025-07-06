use crate::models::{AccountDto, SummonerDto};
use std::io::Write;
use viuer::{Config, print_from_file};

pub fn display_summoner_icon(path: String) {
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

pub fn display_summoner_stats(account: &AccountDto, summoner: &SummonerDto) {
    let name = format!("{}#{}", account.game_name, account.tag_line);
    let level = format!("LvL {}", summoner.summoner_level);

    let spaces = " ".repeat(30 - name.len() - level.len());

    println!("{}{}{}", name, spaces, level);
}
