#[cfg(feature = "talc")]
use talc::{*, source::Claim};
use eframe::NativeOptions;

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

fn main() -> eframe::Result {
    egui_logger::builder().init().unwrap();
    let options = NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
