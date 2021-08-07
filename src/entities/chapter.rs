use crate::uri::Uri;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Chapter {
    // "A Normal Play Time (NMP) timestamp - a single point in time relative to the beginning of
    // the episode audio file." - 5.31
    // TODO: consider replacing with a proper type to represent durations (See Spotify Specs)
    // Time from beginning of episode in seconds
    pub start: i32,
    // "The name of the episode segment." - 5.31
    pub title: String,
    // "Web address of the associated page referred to in the segment." - 5.31
    pub uri: Option<Uri>,
    // "Web address of the associated image referred to in the segment." - 5.31
}
