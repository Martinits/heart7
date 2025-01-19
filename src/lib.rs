pub mod heart7_rpc {
    tonic::include_proto!("heart7_rpc");
}

pub use tonic::{Code, transport::Server, Request, Response, Status};
pub use heart7_rpc::{*, game_msg::*, room_info::*};

pub use log::{debug, error, info, warn};

pub mod rule;

pub use rule::DUMMY_CARD;

pub const DEFAULT_PORT: u16 = 20007;

pub const DEFAULT_CHANNEL_SIZE: usize = 64;

pub mod ui;

pub mod client;
pub mod server;
