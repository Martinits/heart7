pub mod heart7_rpc {
    tonic::include_proto!("heart7_rpc");
}

pub use tonic::{Code, transport::Server, Request, Response, Status};
pub use heart7_rpc::{*, game_msg::*, room_info::*};
use tokio_util::sync::CancellationToken;
use std::panic;
pub use log::{debug, error, info, warn};

pub mod rule;

pub use rule::DUMMY_CARD;

pub const DEFAULT_PORT: u16 = 20007;

pub const DEFAULT_CHANNEL_SIZE: usize = 64;

pub mod tui;

pub mod client;
pub mod server;


pub(crate) fn add_cancel_to_panic(cancel: CancellationToken) {
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        cancel.cancel();
        panic_hook(panic);
    }));
}
