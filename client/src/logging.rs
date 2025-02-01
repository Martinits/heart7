#[cfg(not(target_arch = "wasm32"))]
pub use log::*;

#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! error {
    ($($arg:expr),+) => {
        gloo_console::log!(format!($($arg),+));
    }
}

#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! debug {
    ($($arg:expr),+) => {
        gloo_console::log!(format!($($arg),+));
    }
}

#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! info {
    ($($arg:expr),+) => {
        gloo_console::log!(format!($($arg),+));
    }
}

#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! trace {
    ($($arg:expr),+) => {
        gloo_console::log!(format!($($arg),+));
    }
}

#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! warn {
    ($($arg:expr),+) => {
        gloo_console::log!(format!($($arg),+));
    }
}
