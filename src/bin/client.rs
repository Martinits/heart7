use heart7::tui;
use tui::app::{App, AppResult};
use tui::event::EventHandler;
use tui::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> AppResult<()> {
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
