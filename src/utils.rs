#[macro_export]
macro_rules! debug_reload {
    ($call:expr) => {
        if cfg!(all(debug_assertions, feature = "hotreload")) {
            subsecond::call(|| $call)
        } else {
            $call
        }
    };
}
