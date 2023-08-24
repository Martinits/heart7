use heart7::{*, heart7_server::*};
use std::error::Error;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use clap::Parser;
use heart7::server::*;

#[derive(Parser, Debug)]
#[command(name="Heart7 Server", author="Martinit", about="Heart7 Card Game Server", long_about=None)]
struct Args {
    /// Listen address: <IP>:<PORT>
    #[arg(long, default_value("0.0.0.0:20007"))]
    listen: String,
}

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
        .add_service(Heart7Server::new(server))
        .serve(sock_addr)
        .await?;

    Ok(())
}
