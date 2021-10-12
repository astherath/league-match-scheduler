use super::matches::MatchData;
use icalendar::{Calendar, Component, Event};
use std::{fs, io};

pub fn generate_calendar_event_for_match(matches: &[MatchData]) -> io::Result<()> {
    let mut calendar = Calendar::new();
    matches.iter().for_each(|match_data| {
        let event = event_from_match(match_data);
        calendar.push(event);
    });

    let filename = "/Users/felipearce/Downloads/test.ics";
    let data = calendar.to_string();
    fs::write(filename, &data)?;
    Ok(())
}

fn event_from_match(match_data: &MatchData) -> Event {
    Event::new()
        .starts(match_data.start_timestamp())
        .ends(match_data.end_timestamp())
        .summary(&match_data.summary())
        .description(&match_data.description())
        .done()
}
