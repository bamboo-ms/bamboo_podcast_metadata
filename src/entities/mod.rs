// All quotations (includes submodules) from version of Spotify Podcast Delivery Specification at
// time of writing unless otherwise specified.

pub mod chapter;
pub mod episode;
pub mod show;

// "[Explicit] indicates that the episode contains content not suitable to minors. [Clean] means the
// episode is suitable to minors and [has] not been edited from its original. [Edited] means the
// episode has been edited to become suitable to minors." - 5.27
pub enum Rating {
    Explicit,
    Clean,
    Edited,
}
