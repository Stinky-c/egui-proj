use eframe::NativeOptions;
use std::path::PathBuf;

#[cfg(feature = "talc")]
use talc::{source::Claim, *};

#[cfg(all(feature = "talc", feature = "mimalloc"))]
compile_error!("Only one allocator can be enabled. Pick between 'mimalloc' or 'talc'");

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "talc")]
#[global_allocator]
static TALC: TalcLock<spinning_top::RawSpinlock, Claim> = TalcLock::new(unsafe {
    static mut INITIAL_HEAP: [u8; min_first_heap_size::<DefaultBinning>() + 100000] =
        [0; min_first_heap_size::<DefaultBinning>() + 100000];

    Claim::array(&raw mut INITIAL_HEAP)
});

const APP_NAME: &str = concat!("com.buckydev.", env!("CARGO_PKG_NAME"));
mod app;
pub mod carditems;
pub mod components;

fn main() -> eframe::Result {
    egui_logger::builder().init().unwrap();
    let options = NativeOptions {
        persist_window: true,
        persistence_path: Some(PathBuf::from(
            r#"./config/hello.ron"#,
        )),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
