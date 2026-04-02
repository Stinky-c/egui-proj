use crate::app::App;
use egui::PopupCloseBehavior;
use egui::containers::menu::MenuConfig;
use egui_async::UiExt;

pub(crate) fn menubar(app: &mut App, ui: &mut egui::Ui) {
    egui::MenuBar::new()
        .config(MenuConfig::default().close_behavior(PopupCloseBehavior::CloseOnClickOutside))
        .ui(ui, |ui| {
            menu_debug(app, ui);
            menu_options(app, ui);
        });
}

fn menu_debug(app: &mut App, ui: &mut egui::Ui) {
    ui.menu_button("Debug", |ui| {
        ui.toggle_value(&mut app.show_log, "Show Logs");
        ui.toggle_value(&mut app.show_memory, "Show Memory");
        ui.toggle_value(&mut app.show_loaders, "Show Loaders");
    });
}

fn menu_options(app: &mut App, ui: &mut egui::Ui) {
    ui.menu_button("Options", |ui| {
        if ui.button("Option 1").clicked() {
            ui.popup_notify("Exit app");
        };
    });
}
