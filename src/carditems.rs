use egui::Id;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Hash)]
pub(crate) struct Card {
    pub(crate) image_link: String,
    pub(crate) title: String,
    pub(crate) description: String,
}

impl Card {
    pub(crate) fn blank() -> Self {
        Self {
            image_link: "".to_string(),
            title: "".to_string(),
            description: "".to_string(),
        }
    }
    fn new(
        image_link: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
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
