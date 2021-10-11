use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CompleteMatchData {
    data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    schedule: Schedule,
}

#[derive(Serialize, Deserialize, Debug)]
struct Schedule {
    pages: Pages,
    events: Vec<MatchData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pages {
    older: String,
    newer: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchData {
    start_time: String,
    state: String, // TODO: this should be an enum
    #[serde(rename = "type")]
    match_type: String,
    block_name: String,
    league: League,
    #[serde(rename = "match")]
    match_data: Match,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct League {
    name: String,
    slug: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    id: String,
    teams: Vec<Team>,
    flags: Vec<String>,
    strategy: Strategy,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    name: String,
    code: String,
    image: String,
    result: Option<Result>,
    record: Option<Record>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Result {
    #[serde(rename = "gameWins")]
    game_wins: u32,
    outcome: Option<String>, // TODO: this should be an enu>m
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    wins: u8,
    losses: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Strategy {
    #[serde(rename = "type")]
    match_type: String,
    count: u8,
}
