use crate::carditems::Card;
use crate::components::CardBuilder;
use eframe::{Frame, Storage};
use egui::{Context, Rect, Theme, Ui};
use egui_async::Bind;
use log::{info, trace};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use crate::debug_reload;

const WINDOWS_STORAGE_KEY: &'static str = "show_windows";
const DECK_STORAGE_KEY: &'static str = "deck";

pub(crate) struct App {
    pub(crate) windows: ShowWindowsFlags,
    msg: Bind<String, String>,
    pub(crate) deck_rect: Rect,
    pub(crate) items: Vec<Card>,
    pub(crate) card_builder: CardBuilder,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub(crate) struct ShowWindowsFlags {
    pub(crate) log: bool,
    pub(crate) memory: bool,
    pub(crate) loaders: bool,
    pub(crate) card_builder: bool,
}

impl ShowWindowsFlags {}

impl App {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        let storage = cc.storage.expect("CreationContext Storage is not set?");

        {
            let ctx = cc.egui_ctx.clone();
            subsecond::register_handler(Arc::new(move || ctx.request_repaint()));
        }

        Self {
            windows: get_value_or_default(storage, WINDOWS_STORAGE_KEY),
            msg: Bind::default(),
            deck_rect: Rect::ZERO,
            items: crate::carditems::helper(),
            card_builder: CardBuilder::new(),
        }
    }
}

impl eframe::App for App {
    fn logic(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.plugin_or_default::<egui_async::EguiAsyncPlugin>();
    }

    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        debug_reload!(crate::components::menubar(self, ui));

        if self.windows.log {
            egui::Window::new("Log").show(ui, |ui| {
                egui_logger::logger_ui().show(ui);
            });
        }
        if self.windows.memory {
            let ctx = ui.ctx();
            egui::Window::new("Memory").show(ui, |ui| ctx.memory_ui(ui));
        }

        if self.windows.loaders {
            let ctx = ui.ctx();
            egui::Window::new("Loaders").show(ui, |ui| ctx.loaders_ui(ui));
        }

        if self.windows.card_builder {
            egui::Window::new("Card Builder").show(ui, |ui| {
                debug_reload!(crate::components::cardbuilder(self, ui));
            });
        }

        egui::CentralPanel::default().show_inside(ui, |ui| {
            debug_reload!(crate::components::deckscene(self, ui));
        });
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, WINDOWS_STORAGE_KEY, &self.windows);
        storage.flush();
    }

    fn auto_save_interval(&self) -> Duration {
        Duration::from_mins(1)
    }
}

async fn say_hello() -> Result<String, String> {
    info!("Starting hello");
    tokio::time::sleep(Duration::from_millis(1000)).await;
    info!("Resolved hello");
    Ok("Hello, World!".to_string())
}

fn get_value_or_default<T>(storage: &dyn Storage, key: &str) -> T
where
    T: Default + for<'de> serde::Deserialize<'de>,
{
    eframe::get_value(storage, key).unwrap_or_else(|| {
        trace!("Missing: {}. Assuming default", key);
        T::default()
    })
}
