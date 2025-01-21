use heart7::*;
use heart7::client::Client;
use tui::term_event::TermEventHandler;
use std::env;
use tokio_util::sync::CancellationToken;
use tokio::sync::mpsc;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    #[clap(default_value_t=format!("127.0.0.1:{}", DEFAULT_PORT))]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let logfile = match env::var("LOGFILE") {
        Ok(f) => f,
        Err(_) => "/dev/null".into()
    };

    let logfile = FileAppender::builder().build(logfile)?;
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Debug))?;
    log4rs::init_config(config)?;

    info!("Heart7 Client Starts!");

    let cancel = CancellationToken::new();

    let (tx, rx) = mpsc::channel(DEFAULT_CHANNEL_SIZE);
    let mut client = Client::new(cancel.clone(), tx.clone(), rx, args.addr).await?;

    let te = TermEventHandler::new();
    info!("Starting terminal event handler...");
    te.run(DEFAULT_CHANNEL_SIZE, &cancel, tx)?;

    info!("Starting main task...");
    // main task: state manager + render task + rpc client
    client.run().await?;

    info!("Exiting...");
    client.exit()?;

    Ok(())
}
