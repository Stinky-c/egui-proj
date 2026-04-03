/*
#[cfg(feature = "talc")]
use talc::{source::Claim, *};

#[cfg(all(feature = "talc", feature = "mimalloc"))]
compile_error!("Only one allocator can be enabled. Pick between 'mimalloc' or 'talc'");

#[cfg(all(feature = "mimalloc",not(target_arch = "wasm32")))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(feature = "talc",not(target_arch = "wasm32")))]
#[global_allocator]
static TALC: TalcLock<spinning_top::RawSpinlock, Claim> = TalcLock::new(unsafe {
    static mut INITIAL_HEAP: [u8; min_first_heap_size::<DefaultBinning>() + 100000] =
        [0; min_first_heap_size::<DefaultBinning>() + 100000];

    Claim::array(&raw mut INITIAL_HEAP)
});
 */

const APP_NAME: &str = concat!("com.buckydev.", env!("CARGO_PKG_NAME"));
mod app;
pub mod carditems;
pub mod components;
mod utils;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    use std::path::PathBuf;
    egui_logger::builder().init().unwrap();

    #[cfg(debug_assertions)]
    dioxus_devtools::connect_subsecond();

    let options = eframe::NativeOptions {
        persist_window: true,
        persistence_path: Some(PathBuf::from(r#"./config/hello.ron"#)),
        ..Default::default()
    };

    debug_reload!({
        eframe::run_native(
            APP_NAME,
            options.clone(),
            Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
        )
    })
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    // eframe::WebLogger::init(log::LevelFilter::Debug).expect("Failed to initialize web logger");
    egui_logger::builder()
        .init()
        .expect("failed to initialize egui logger");

    let web_options = eframe::WebOptions {
        ..Default::default()
    };

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
