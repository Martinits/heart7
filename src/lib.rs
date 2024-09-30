pub mod heart7_rpc {
    tonic::include_proto!("heart7_rpc");
}

pub use tonic::{Code, transport::Server, Request, Response, Status};
pub use heart7_rpc::{*, game_msg::*, room_info::*};

pub use log::{debug, error, info, warn};

pub mod rule;

pub use rule::DUMMY_CARD;

pub type RPCResult<T> = Result<T, tonic::Status>;

use rule::game::GameError;

impl From<GameError> for tonic::Status {
    fn from(value: GameError) -> Self {
        match value {
            GameError::PermissionDenied(s) => Self::permission_denied(s),
            GameError::NotFound(s) => Self::not_found(s),
            GameError::AlreadyDone(s) => Self::already_exists(s),
            GameError::Internal(s) => Self::internal(s),
        }
    }
}

pub const DEFAULT_PORT: u16 = 20007;

pub const DEFAULT_CHANNEL_SIZE: usize = 64;

pub mod tui;

pub mod client;
pub mod server;
