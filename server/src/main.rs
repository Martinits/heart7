mod room;
mod server;

use server::*;
use heart7_rule::*;
use tonic::transport::Server;
use std::error::Error;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use clap::Parser;
use log::*;
pub use tonic::{Code, Request, Response, Status};

pub const DEFAULT_PORT: u16 = 20007;

pub const DEFAULT_CHANNEL_SIZE: usize = 64;

pub type RPCResult<T> = Result<T, tonic::Status>;

#[derive(Parser, Debug)]
#[command(name="Heart7 Server", author="Martinit", about="Heart7 Card Game Server", long_about=None)]
struct Args {
    /// Listen address: <IP>:<PORT>
    #[arg(long, default_value("0.0.0.0:20007"))]
    listen: String,
}

// const DEFAULT_MAX_AGE: Duration = Duration::from_secs(24 * 60 * 60);
// const DEFAULT_EXPOSED_HEADERS: [&str; 3] =
//     ["grpc-status", "grpc-message", "grpc-status-details-bin"];
// const DEFAULT_ALLOW_HEADERS: [&str; 4] =
//     ["x-grpc-web", "content-type", "x-user-agent", "grpc-timeout"];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let logconsole = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(logconsole)))
        .build(Root::builder()
                   .appender("console")
                   .build(LevelFilter::Debug))?;
    log4rs::init_config(config)?;

    let args = Args::parse();


    let sock_addr = args.listen.parse()?;
    let server = Heart7D::default();
    server.spawn_watch_dog();

    info!("Heart7 Server serving on {}..", sock_addr);
    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(Heart7Server::new(server)))
        // .layer(
        //     CorsLayer::new()
        //         .allow_origin(AllowOrigin::mirror_request())
        //         .allow_credentials(true)
        //         .max_age(DEFAULT_MAX_AGE)
        //         .expose_headers(
        //             DEFAULT_EXPOSED_HEADERS
        //                 .iter()
        //                 .cloned()
        //                 .map(HeaderName::from_static)
        //                 .collect::<Vec<HeaderName>>(),
        //         )
        //         .allow_headers(
        //             DEFAULT_ALLOW_HEADERS
        //                 .iter()
        //                 .cloned()
        //                 .map(HeaderName::from_static)
        //                 .collect::<Vec<HeaderName>>(),
        //         ),
        // )
        // .layer(GrpcWebLayer::new())
        // .add_service(Heart7Server::new(server))
        .serve(sock_addr)
        .await?;

    Ok(())
}
