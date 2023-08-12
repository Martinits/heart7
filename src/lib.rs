pub mod heart7_rpc {
    tonic::include_proto!("heart7_rpc");
}

pub use tonic::{Code, transport::Server, Request, Response, Status};
pub use heart7_rpc::{*, game_msg::*};

pub mod room;
pub mod game;

pub type RPCResult<T> = Result<T, tonic::Status>;

pub const DEFAULT_PORT: u16 = 20007;
