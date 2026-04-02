use serde::{Deserialize, Serialize};

/// Expected to be a valid card
#[derive(Serialize, Deserialize, Hash)]
pub(crate) struct Card {
    pub(crate) image_link: String,
    pub(crate) title: String,
    pub(crate) description: String,
}

impl Card {
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
