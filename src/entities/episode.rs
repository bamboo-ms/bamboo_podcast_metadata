use super::{chapter::Chapter, Rating};
use bamboo_metadata::id_provider::IdProvider;
use bamboo_metadata::title::Title;
use chrono::{Duration, NaiveDate};
use http::Uri;

// Contains metadata about episode file content, including URI
// TODO: look into rss:enclosure instead
pub struct Enclosure {
    // "Specifies the location of the episode audio file." - 5.17
    pub uri: Uri,
    // TODO: Is type needed if only audio/mpeg is supported?
    // "The length of the audio file in bytes." - 5.17
    pub length: Option<i64>,
}

pub enum EpisodeId {
    Link(Uri),
    Guid(String),
}

impl IdProvider for EpisodeId {
    fn database_id(&self) -> String {
        "feed".to_string()
    }

    fn database_value(&self) -> String {
        match self {
            EpisodeId::Link(uri) => uri.to_string(),
            EpisodeId::Guid(s) => s.clone(),
        }
    }
}

pub struct Episode {
    // "Unique provider identifier for the podcast which is stable over time." - 5.16
    pub id: EpisodeId,
    // TODO: Is the enclosure used for anything?
    // "This is the default RSS definition of the episode content file. This is an extension to
    // <enclosure> tag and is mostly used for video files." - 5.17/5.23
    // To reduce confusion with the ambiguous name "Media," enclosure is used to represent data
    // from both the <enclosure> and <media:content> tags.
    pub enclosure: Enclosure,
    // "Indicates the age of the episode when the episode was first published in any outlet... This
    // value is used to order episodes when no other explicit order is specified." - 5.18
    pub published: Option<NaiveDate>,
    // "The title of the specific podcast episode..." - 5.19/5.20
    pub title: Title,
    // "Phrase or short example of the episode to give the consumer a quick understanding of its
    // content..." - 5.21/5.22
    pub description: String,
    // MEDIA:RESTRICTION (5.24) is intentionally omitted as it is not applicable to the Bamboo side
    // of distribution - content restriction should be enforced at the publisher level.
    pub duration: Option<Duration>,
    // "Indicates the order in which the episode shall be played. If omitted, the [published] field
    // will be used to generate order with the oldest episode starting as the first list entry." -
    // 5.26
    pub order: Option<usize>,
    // "Indicates if the podcast episode contains explicit material..." - 5.27
    pub rating: Option<Rating>,
    // "Indicates episode artwork. Should be a 1:1 dimensioned image in high resolution
    // representing the particular episode. When omitted on the item level the [] channel level
    // will be used instead." - 5.28
    pub image: Option<Uri>,
    // 5.29 RSS/CHANNEL/ITEM/DCTERMS:VALID is intentionally omitted. Access to streaming podcasts
    //   should be enforced at the filesystem level.

    // "Creates a list of chapters for this episode. If present there must be at least one chapter
    // defined in the list." - 5.30
    pub chapters: Vec<Chapter>,
}
