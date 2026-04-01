use eframe::Frame;
use egui::{Context, Ui};
use egui_async::Bind;
use log::info;

pub(crate) struct App {
    show_log: bool,
    msg: egui_async::Bind<String, String>,
}

impl App {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            show_log: cfg!(debug_assertions),
            msg: Bind::default(),
        }
    }
}

impl eframe::App for App {
    fn logic(&mut self, ctx: &Context, frame: &mut Frame) {
        ctx.plugin_or_default::<egui_async::EguiAsyncPlugin>();
    }

    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        if self.show_log {
            egui::Window::new("Log").show(ui, |ui| {
                egui_logger::logger_ui().show(ui);
            });
        }
        egui::CentralPanel::default().show_inside(ui, |ui| {
            if let Some(msg) = self.msg.read_or_request(say_hello) {
                match msg {
                    Ok(v) => {
                        ui.label(v);
                    }
                    Err(err) => {
                        ui.label(err);
                    }
                }
            }
        });
    }
}

async fn say_hello() -> Result<String, String> {
    info!("Starting hello");
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    info!("Resolved hello");
    Ok("Hello, World!".to_string())
}
