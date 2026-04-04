use crate::app::App;
use crate::carditems::Card;
use egui::{Color32, RichText, Ui, WidgetText};
use egui_async::UiExt;
use log::{error, info, trace};
use std::mem::take;

/// Does not need to be a valid card, but does need to follow [crate::carditems::Card].
/// `title`, `description`, and `image_link` are buffers for line editors.
/// `is_dirty` denotes if buffers have been changed since last validation.
/// `error` represents being invalid and what error.
pub(crate) struct CardBuilder {
    is_dirty: bool,
    error: Option<CardBuilderError>,
    // Buffers
    title: String,
    description: String,
    image_link: String,
}
#[derive(Copy, Clone, Debug)]
enum CardBuilderError {
    BadImageLink,
    BadTitle,
    BadDescription,
}
impl CardBuilderError {
    fn msg(&self) -> &str {
        match self {
            CardBuilderError::BadImageLink => "Bad image link",
            CardBuilderError::BadTitle => "Bad title",
            CardBuilderError::BadDescription => "Bad description",
        }
    }
}
impl CardBuilder {
    pub(crate) fn new() -> Self {
        Self {
            is_dirty: true,
            error: None,
            title: String::new(),
            image_link: String::new(),
            description: String::new(),
        }
    }
    fn mark_dirty(&mut self) {
        self.is_dirty = true
    }

    /// Check if all fields are valid. Returns `()` if valid. Errors if not valid.
    /// Called every frame so
    fn validate(&mut self) -> Result<(), CardBuilderError> {
        // Return early if clean and have error. Means nothing has changed since last validation
        if let Some(v) = self.error
            && !self.is_dirty
        {
            trace!("Card validation return early");
            return Err(v);
        }

        // If empty, or not ascii
        if self.title.is_empty() || !self.title.is_ascii() {
            error!("Card title is empty or non-ASCII");
            self.error = Some(CardBuilderError::BadTitle);
        }

        // if empty, or not ascii, or not url
        if self.image_link.is_empty() || !self.image_link.is_ascii() {
            error!("Card image link is empty or non-ASCII");
            self.error = Some(CardBuilderError::BadImageLink);
        }
        if let Err(e) = url::Url::parse(&self.image_link) {
            error!("Card image link parse error: {}", e);
            self.error = Some(CardBuilderError::BadImageLink);
        }

        // If empty, or not ascii
        if self.description.is_empty() || !self.description.is_ascii() {
            error!("Card description is empty or non-ASCII");
            self.error = Some(CardBuilderError::BadDescription);
        }

        self.is_dirty = false;
        if let Some(e) = self.error {
            error!("Card validation failed with error: {:?}", e);
            Err(e)
        } else {
            info!("Card validation passed");
            Ok(())
        }
    }

    fn clear_error(&mut self) {
        self.error = None;
        self.is_dirty = true;
    }

    fn finalize(&mut self) -> Result<Card, CardBuilderError> {
        if let Err(e) = self.validate() {
            return Err(e);
        }

        self.clear_error();

        let link = take(&mut self.image_link)
            .parse()
            .expect("Failed to parse image_link. Validation errored on finalize");
        let card = Card::new(link, take(&mut self.title), take(&mut self.description));

        Ok(card)
    }
}

pub(crate) fn cardbuilder(app: &mut App, ui: &mut Ui) {
    ui.heading("Card Builder");
    ui.group(|ui| {
        if edit_line_with_label(ui, "Title", &mut app.card_builder.title)
            || edit_line_with_label(ui, "Image Link", &mut app.card_builder.image_link)
            || edit_multiline_with_label(ui, "Description", &mut app.card_builder.description)
        {
            app.card_builder.mark_dirty()
        }
    });

    ui.horizontal(|ui| {
        if ui.button("Validate").clicked() {
            let _ = app.card_builder.validate();
        };

        ui.add_enabled_ui(
            !app.card_builder.is_dirty && app.card_builder.error.is_none(),
            |ui| {
                if ui.button("Finalize").clicked() {
                    // oops i lost it
                    // TODO: Copy successful card creation into deck
                    info!("Card builder finalized");
                    match app.card_builder.finalize() {
                        Ok(v) => app.items.push(v),
                        Err(e) => {
                            info!("Card builder failed with error: {:?}", e);
                        }
                    }
                }
            },
        );
    });

    if let Some(err) = app.card_builder.error
        && !app.card_builder.is_dirty
    {
        ui.separator();
        ui.heading(RichText::new("Error").color(Color32::RED));
        ui.label(err.msg());
    }
    // Card must not have been changed since last validation, and must not have any errors
}

fn edit_line_with_label(ui: &mut Ui, label: impl Into<WidgetText>, buf: &mut String) -> bool {
    ui.label(label);
    ui.text_edit_singleline(buf).changed()
}

fn edit_multiline_with_label(ui: &mut Ui, label: impl Into<WidgetText>, buf: &mut String) -> bool {
    ui.label(label);
    ui.text_edit_multiline(buf).changed()
}
