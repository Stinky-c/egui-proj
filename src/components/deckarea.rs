use crate::app::App;
use egui::{Id, Rect, Scene, ScrollArea, Ui};
use egui_dnd::{DragDropItem, dnd};
use std::any::Any;

pub(crate) fn deckscene(app: &mut App, ui: &mut Ui) {
    let area = ScrollArea::vertical()
        .id_salt("deckscene")
        .auto_shrink(false);

    area.show(ui, |ui| {
        deckarea(app, ui);
    });
}

fn deckarea(app: &mut App, ui: &mut Ui) {
    dnd(ui, "dnd_example").show_vec(&mut app.items, |ui, item, handle, state| {
        ui.horizontal(|ui| {
            handle.ui(ui, |ui| {
                if state.dragged {
                    ui.label("dragging");
                } else {
                    ui.label("drag");
                }
            });
            ui.label(item.title.as_str());
        });
    });
}
