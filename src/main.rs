use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::dbg;
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    let env_key = "RIOT_API_KEY";
    let fetcher = DataFetcher::new(env_key);

    let leagues = fetcher.get_all_leagues().await;

    println!("Hello, world!");
}

struct DataFetcher {
    http_client: Client,
    // api_key: String
}

#[derive(Serialize, Deserialize, Debug)]
struct RawLeagueData {
    data: LeagueDataArray,
}

#[derive(Serialize, Deserialize, Debug)]
struct LeagueDataArray {
    leagues: Vec<LeagueData>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct LeagueData {
    id: String,
    slug: String,
    name: String,
    region: String,
    image: String,
    priority: i32,
    displayPriority: DisplayPriority,
}

#[derive(Serialize, Deserialize, Debug)]
struct DisplayPriority {
    position: i32,
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Match;

impl DataFetcher {
    fn new(env_key: &str) -> Self {
        let api_key = get_api_key(env_key);
        let http_client = http::create_http_client(&api_key);
        Self { http_client }
    }

    // fn get_league_by_name(&self, league_name: &str) -> LeagueData {
    // }

    async fn get_all_leagues(&self) -> Vec<LeagueData> {
        let endpoint = "persisted/gw/getLeagues";
        let params = [("hl", "en-US")];

        let url = http::get_url_for_endpoint_with_params(&endpoint, &params);
        let data = http::get_data_from_url::<LeagueDataArray>(&self.http_client, url).await;
        dbg!(&data);

        vec![]
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
        client.get(url).send().await.unwrap().json().await.unwrap()
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
        let base_url_str = "https://esports-api.lolesports.com";
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
