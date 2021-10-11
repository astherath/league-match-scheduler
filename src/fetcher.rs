use super::http;
use super::leagues::{LeagueData, LeagueDataArray, RawLeagueData};
use super::matches::CompleteMatchData;
use reqwest::Client;

pub struct DataFetcher {
    http_client: Client,
}

impl DataFetcher {
    pub fn new() -> Self {
        let http_client = http::create_http_client();
        Self { http_client }
    }

    pub async fn get_matches_for_league(&self, league: LeagueData) -> CompleteMatchData {
        let league_id = league.id;
        let endpoint = "getSchedule";
        let params = [("hl", "en-US"), ("leagueId", &league_id)];

        let url = http::get_url_for_endpoint_with_params(&endpoint, &params);
        let data = http::get_data_from_url(&self.http_client, url).await;

        data
    }

    pub async fn get_league_by_name(&self, league_name: &str) -> Option<LeagueData> {
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
}
