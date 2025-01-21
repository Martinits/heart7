mod game;
mod rpc;
pub use game::*;
pub use rpc::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

pub(crate) type JsResult<T> = Result<T, JsValue>;

#[wasm_bindgen(start)]
fn main() {
    spawn_local(test_rpc());

    start_game().unwrap_throw();
}
