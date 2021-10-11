use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawLeagueData {
    pub data: LeagueDataArray,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueDataArray {
    pub leagues: Vec<LeagueData>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LeagueData {
    pub id: String,
    slug: String,
    pub name: String,
    region: String,
    image: String,
    priority: i32,
    #[serde(rename = "displayPriority")]
    display_priority: DisplayPriority,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct DisplayPriority {
    position: i32,
    status: String,
}
