use serde::{Deserialize, Serialize};
use url::Url;

/// Expected to be a valid card
#[derive(Serialize, Deserialize, Hash)]
pub(crate) struct Card {
    pub(crate) image_link: Url,
    pub(crate) title: String,
    pub(crate) description: String,
}

impl Card {
    fn new(image_link: Url, title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            image_link: image_link.into(),
            title: title.into(),
            description: description.into(),
        }
    }
}

pub(crate) fn helper() -> Vec<Card> {
    ron::from_str(include_str!("../config/cards.ron")).unwrap()
}
