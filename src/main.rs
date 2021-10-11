use tokio;
mod fetcher;
mod http;
mod leagues;
mod matches;
use fetcher::DataFetcher;

#[tokio::main]
async fn main() {
    let fetcher = DataFetcher::new();

    let league_wanted = "worlds";

    let league = fetcher
        .get_league_by_name(league_wanted)
        .await
        .expect("league data should have been succesfully fetched");
    let matches = fetcher.get_matches_for_league(league).await;
    dbg!(matches);
}
