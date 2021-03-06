use chrono::{DateTime, Duration, Local, NaiveDateTime, TimeZone};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct CompleteMatchData {
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub schedule: Schedule,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Schedule {
    pages: Pages,
    pub events: Vec<MatchData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pages {
    older: String,
    newer: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct MatchData {
    pub start_time: String,
    pub state: String,
    #[serde(rename = "type")]
    pub match_type: String,
    pub block_name: String,
    pub league: League,
    #[serde(rename = "match")]
    pub match_data: Match,
}

impl MatchData {
    pub fn start_timestamp(&self) -> NaiveDateTime {
        let mut date_time = DateTime::parse_from_rfc3339(&self.start_time)
            .unwrap()
            .naive_utc();
        let offset = Local::offset_from_utc_datetime(&Local, &date_time);
        let offset_in_secs = Duration::seconds(offset.local_minus_utc() as i64);
        date_time = date_time + offset_in_secs;
        date_time
    }

    pub fn end_timestamp(&self) -> NaiveDateTime {
        let start = self.start_timestamp();
        let end = Duration::hours(self.match_data.strategy.count as i64);
        start + end
    }

    pub fn summary(&self) -> String {
        format!("{}", self.match_data.summary())
    }

    pub fn description(&self) -> String {
        format!("{}", self.match_data.description())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct League {
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Match {
    pub id: String,
    pub teams: Vec<Team>,
    pub flags: Vec<String>,
    pub strategy: Strategy,
}

impl Match {
    fn summary(&self) -> String {
        format!("{} vs {}", self.teams[0].code, self.teams[1].code)
    }
    fn description(&self) -> String {
        format!("{} | {}", self.teams[0], self.teams[1])
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Team {
    pub name: String,
    pub code: String,
    pub image: String,
    pub result: Option<Result>,
    pub record: Option<Record>,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let record_str = match &self.record {
            Some(record) => record.to_string(),
            None => "".to_string(),
        };
        write!(f, "{} {}", &self.name, record_str)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Result {
    #[serde(rename = "gameWins")]
    pub game_wins: u32,
    pub outcome: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Record {
    pub wins: u8,
    pub losses: u8,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}-{})", &self.wins, &self.losses)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Strategy {
    #[serde(rename = "type")]
    pub match_type: String,
    pub count: u8,
}
