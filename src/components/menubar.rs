use crate::app::App;
use egui::PopupCloseBehavior;
use egui::containers::menu::MenuConfig;
use log::{error, info};

pub(crate) fn menubar(app: &mut App, ui: &mut egui::Ui) {
    egui::MenuBar::new()
        .config(MenuConfig::default().close_behavior(PopupCloseBehavior::CloseOnClickOutside))
        .ui(ui, |ui| {
            menu_deck(app, ui);
            menu_debug(app, ui);
            menu_options(app, ui);
        });
}

fn menu_debug(app: &mut App, ui: &mut egui::Ui) {
    ui.menu_button("Debug", |ui| {
        ui.toggle_value(&mut app.windows.log, "Show Logs");
        ui.toggle_value(&mut app.windows.memory, "Show Memory");
        ui.toggle_value(&mut app.windows.loaders, "Show Loaders");
    });
}

fn menu_options(app: &mut App, ui: &mut egui::Ui) {
    ui.menu_button("Options", |ui| {
        if ui.button("Option 1").clicked() {
            error!("Option 1");
        };
    });
}

fn menu_deck(app: &mut App, ui: &mut egui::Ui) {
    ui.menu_button("Deck", |ui| {
        ui.toggle_value(&mut app.windows.card_builder, "Card Builder");

        if ui.button("Save").clicked() {
            let obj = ron::to_string(&app.items).unwrap();
            info!("{obj}")

            // TODO: open file dialog, save deck to file. Blocking here is fine
        }
        if ui.button("Load").clicked() {
            // TODO: open file dialog, load deck from file. Blocking here is fine
        }
    });
}
