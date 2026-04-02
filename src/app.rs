use crate::carditems::Card;
use eframe::Frame;
use egui::{Context, Rect, Scene, Ui};
use egui_async::Bind;
use log::info;

pub(crate) struct App {
    pub(crate) show_log: bool,
    pub(crate) show_memory: bool,
    msg: Bind<String, String>,
    pub(crate) deck_rect: Rect,
    pub(crate) items: Vec<Card>,
    pub(crate) show_loaders: bool,
    pub(crate) card_builder: Card,
}

impl App {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        Self::default()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            show_log: false,
            show_memory: false,
            show_loaders: false,
            msg: Bind::default(),
            deck_rect: Rect::ZERO,
            items: crate::carditems::helper(),
            card_builder: Card::blank()
        }
    }
}

impl eframe::App for App {
    fn logic(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.plugin_or_default::<egui_async::EguiAsyncPlugin>();
    }

    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        crate::components::menubar(self, ui);

        if self.show_log {
            egui::Window::new("Log").show(ui, |ui| {
                egui_logger::logger_ui().show(ui);
            });
        }
        if self.show_memory {
            egui::Window::new("Memory").show(ui, |ui| ui.label("Memory go here"));
        }

        if self.show_loaders {
            egui::Window::new("Loaders").show(ui, |ui| ui.label("Loaders go here"));
        }

        egui::Window::new("Card Builder").show(ui, |ui| {
            ui.group(|ui| {
                ui.text_edit_singleline(&mut self.card_builder.title);
                ui.text_edit_singleline(&mut self.card_builder.image_link);
                ui.text_edit_singleline(&mut self.card_builder.description);
            })
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            crate::components::deckscene(self, ui);
        });
    }
}

async fn say_hello() -> Result<String, String> {
    info!("Starting hello");
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    info!("Resolved hello");
    Ok("Hello, World!".to_string())
}
