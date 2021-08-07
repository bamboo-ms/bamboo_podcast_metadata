use super::episode::Episode;
use super::Rating;
use crate::uri::Uri;
use anyhow::Error;
use bamboo_metadata::language::Language;
use bamboo_metadata::person::Person;
use bamboo_metadata::title::Title;
use isocountry::CountryCode;
use rss::Channel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Chronology {
    Episodic,
    Serial,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    // TODO: Replace with enum of supported categories
    pub name: String,
    pub subcategories: Vec<Category>,
}

// "Indicates a show i.e. the podcast itself. It contains the metadata for the show and
// encapsulates all the show episodes as items. Defining multiple podcasts in the same RSS is not
// supported." - 5.2
#[derive(Serialize, Deserialize)]
pub struct Show {
    // "The name by which the podcast is known." - 5.3
    pub title: Title,
    // "The web address of the podcast Rshttps://podcasters.spotify.com/terms/Spotify_Podcast_Delivery_Specification_v1.6.pdfs page..." 5.4
    pub link: Uri,
    // "Phrase or short description of the podcast intending to give the consumer a quick
    // understanding of what the podcast is about." - 5.5
    pub description: Option<String>,
    // "Indicates the primary show language..." - 5.6
    pub language: Language,
    // "The full name of the show main originator, group or person." - 5.7
    pub author: Option<Person<String, String>>,
    // "Indicates the podcast artwork. This image will be used as a fallback for any podcast
    // episode not defining its own artwork." - 5.8
    pub image: Option<Uri>,
    // TODO: consider reworking the "Rating" type to better accommodate the
    // "Indicates if the podcast contains explicit material in any of its episodes... Clean
    // indicates the podcast in its entirety is suitable to minors with or without edited
    // material. [Explicit] indicates that parental guidance is recommended to parts of the
    // content." - 5.9
    pub rating: Option<Rating>,
    // "Used to group the podcast into specific sets. May be nested with subcategories like 1. Arts
    // -> 1.1 Design, 1.2 Fashion & Beauty, 1.3[] Food[,] 1.4 Literature etc..." 5.10
    pub categories: Vec<Category>,
    // "... A [true] indicates that a podcast has ended and no further episodes will be published."
    // - 5.11
    pub complete: Option<bool>,
    // "... [Episodic] for non-chronological episodes that will behave as they have for years and
    // download the latest episode, or [Serial] for chronological episodes that should be consumed
    // oldest to newest." - 5.12
    pub chronology: Option<Chronology>,
    // NOT IMPLEMENTED: 5.13 RSS/CHANNEL/SPOTIFY:LIMIT
    // "Defines the intended market/territory where the podcast is relevant to the consumer...
    // [C]ountry codes ranked in order of priority from most relevant to least relevant. Podcasts
    // with a narrow list of countries will have a higher potential reaching [its]
    // 5.14
    pub country_of_origin: Vec<CountryCode>,
    pub episodes: Vec<Episode>,
}

impl Show {
    pub fn from_channel(channel: Channel) -> Result<Self, Error> {
        todo!()
    }
}
