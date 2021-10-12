use tokio;
mod calendar;
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
        .expect("error fetching league data");
    let mut matches = fetcher
        .get_matches_for_league(league)
        .await
        .into_iter()
        // get uncompleted matches
        .filter(|x| x.state != "completed")
        // get groups matches
        .filter(|x| x.block_name == "Groups")
        // sort by start date
        .collect::<Vec<matches::MatchData>>();
    matches.sort();

    calendar::generate_calendar_event_for_match(&matches).expect("error creating calendar file");
}
