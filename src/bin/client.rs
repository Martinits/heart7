use heart7::*;
use tui::app::{App, AppResult};
use tui::event::EventHandler;
use tui::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tokio_util::sync::CancellationToken;
use tokio::sync::mpsc;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};

#[tokio::main]
async fn main() -> AppResult<()> {

    let logfile = FileAppender::builder().build("heart7.log")?;
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
    let tui = Tui::new(terminal);

    let (tx, rx) = mpsc::channel(DEFAULT_CHANNEL_SIZE);
    let mut app = App::new(tui, &cancel, tx.clone(), rx).await;
    app.init()?;

    let event = EventHandler::new();
    info!("Starting event handler...");
    event.run(DEFAULT_CHANNEL_SIZE, &cancel, tx)?;

    info!("Starting main task...");
    // main task: state manager + render task + rpc client
    app.run().await?;

    info!("Exiting...");
    app.exit()?;

    Ok(())
}
