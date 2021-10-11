use matches::CompleteMatchData;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    let env_key = "RIOT_API_KEY";
    let fetcher = DataFetcher::new(env_key);

    let league_wanted = "worlds";

    let league = fetcher
        .get_league_by_name(league_wanted)
        .await
        .expect("league data should have been succesfully fetched");
    let matches = fetcher.get_matches_for_league(league).await;
    dbg!(matches);
}

struct DataFetcher {
    http_client: Client,
}

#[derive(Serialize, Deserialize, Debug)]
struct RawLeagueData {
    data: LeagueDataArray,
}

#[derive(Serialize, Deserialize, Debug)]
struct LeagueDataArray {
    leagues: Vec<LeagueData>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct LeagueData {
    id: String,
    slug: String,
    name: String,
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

mod matches {
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
        block_name: Option<String>,
        league: League,
        #[serde(rename = "match")]
        match_data: Option<Match>,
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
        outcome: String, // TODO: this should be an enum
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
}

impl DataFetcher {
    fn new(env_key: &str) -> Self {
        let api_key = get_api_key(env_key);
        let http_client = http::create_http_client(&api_key);
        Self { http_client }
    }

    async fn get_league_by_name(&self, league_name: &str) -> Option<LeagueData> {
        let needle = league_name.to_string().to_lowercase();
        let leagues = self.get_all_leagues().await;
        leagues
            .leagues
            .into_iter()
            .find(|x| x.name.to_lowercase() == needle)
    }

    async fn get_all_leagues(&self) -> LeagueDataArray {
        let endpoint = "getLeagues";
        let params = [("hl", "en-US")];

        let url = http::get_url_for_endpoint_with_params(&endpoint, &params);
        let data = http::get_data_from_url::<RawLeagueData>(&self.http_client, url).await;
        data.data
    }

    async fn get_matches_for_league(&self, league: LeagueData) -> CompleteMatchData {
        let league_id = league.id;
        let endpoint = "getSchedule";
        let params = [("hl", "en-US"), ("leagueId", &league_id)];

        let url = http::get_url_for_endpoint_with_params(&endpoint, &params);
        let data = http::get_data_from_url(&self.http_client, url).await;
        dbg!(&data);

        data
    }
}

mod http {
    use reqwest::header::{HeaderMap, HeaderValue};
    use reqwest::{Client, Url};
    use serde::{Deserialize, Serialize};

    type ParamPairs<'a> = &'a [(&'a str, &'a str)];
    type UrlString = String;

    pub async fn get_data_from_url<T>(client: &Client, url: UrlString) -> T
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        dbg!(client.get(&url).send().await.unwrap().text().await.unwrap());
        client.get(&url).send().await.unwrap().json().await.unwrap()
    }

    pub fn get_url_for_endpoint_with_params(endpoint: &str, params: ParamPairs) -> UrlString {
        let mut base_url = base_url().join(endpoint).unwrap();
        let mut url_with_params = base_url.query_pairs_mut();
        url_with_params.clear();

        params.iter().for_each(|x| {
            url_with_params.append_pair(&x.0, &x.1);
        });

        url_with_params.finish().as_str().to_string()
    }

    fn base_url() -> Url {
        let base_url_str = "https://esports-api.lolesports.com/persisted/gw/";
        Url::parse(base_url_str).unwrap()
    }

    pub fn create_http_client(api_key: &str) -> Client {
        let headers = get_default_headers(api_key);
        Client::builder().default_headers(headers).build().unwrap()
    }

    fn get_default_headers(api_key: &str) -> HeaderMap {
        let x_api_key = HeaderValue::from_str(api_key).unwrap();

        let mut headers = HeaderMap::new();
        headers.insert("X-API-KEY", x_api_key);

        headers
    }
}

fn get_api_key(env_key: &str) -> String {
    env::var(env_key).expect(&format!("riot games API key (\"{}\") not set!", env_key))
}
