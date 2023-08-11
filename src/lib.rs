pub mod heart7_rpc {
    tonic::include_proto!("heart7_rpc");
}

pub use tonic::{transport::Server, Request, Response, Status};
pub use heart7_rpc::{*, heart7_server::*, game_msg::*};

pub mod room;
pub mod game;
