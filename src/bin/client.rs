use heart7::tui;
use tui::app::{App, AppResult};
use tui::event::EventHandler;
use tui::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tokio_util::sync::CancellationToken;
use log::LevelFilter;
use log::{info, debug, warn, error};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};

#[tokio::main]
async fn main() -> AppResult<()> {

    let logfile = FileAppender::builder().build("heart7.log")?;
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Info))?;
    log4rs::init_config(config)?;

    info!("Heart7 Client Starts!");

    let cancel = CancellationToken::new();

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    tui.init(&cancel)?;

    let event = EventHandler::new();
    event.run(64, &cancel)?;

    let mut app = App::new();

    // main task: state manager + render task + rpc client
    app.run().await?;

    // cancel all spawned tasks
    cancel.cancel();
    tui.exit()?;

    Ok(())
}
