use heart7::*;
use heart7::client::{App, AppResult};
use tui::term_event::TermEventHandler;
use tui::tui::Tui;
use std::io;
use std::env;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tokio_util::sync::CancellationToken;
use tokio::sync::mpsc;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    #[clap(default_value_t=format!("127.0.0.1:{}", DEFAULT_PORT))]
    addr: String,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let args = Args::parse();

    let logfile = match env::var("LOGFILE") {
        Ok(f) => f,
        Err(_) => "heart7.log".into()
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

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let sz = terminal.size()?;
    let tui = Tui::new(terminal);

    let (tx, rx) = mpsc::channel(DEFAULT_CHANNEL_SIZE);
    let mut app = App::new(tui, &cancel, tx.clone(), rx, sz, args.addr).await;
    app.init()?;

    let te = TermEventHandler::new();
    info!("Starting event handler...");
    te.run(DEFAULT_CHANNEL_SIZE, &cancel, tx)?;

    info!("Starting main task...");
    // main task: state manager + render task + rpc client
    app.run().await?;

    info!("Exiting...");
    app.exit()?;

    Ok(())
}
