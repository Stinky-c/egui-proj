use crate::app::App;
use egui::{AtomExt, ScrollArea, Ui, Widget, vec2};
use egui_dnd::dnd;

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
            // handle.ui(ui, |ui| {
            //     if state.dragged {
            //         ui.label("dragging");
            //     } else {
            //         ui.label("drag");
            //     }
            // });

            egui::Image::new(egui::include_image!("../../.config/testing.png"))
                .fit_to_original_size(1f32)
                .ui(ui);
        });
    });
}
