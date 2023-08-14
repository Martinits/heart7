use heart7::{*, heart7_client::*};

use tui::app::{App, AppResult};
use tui::event::{Event, EventHandler};
use tui::handler::handle_key_events;
use tui::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_ip = "127.0.0.1";

    let mut client = Heart7Client::connect(
        format!("http://{}:{}", server_ip, DEFAULT_PORT)
    ).await?;

    let request = tonic::Request::new(PlayerInfo {
        name: "Martinits".into(),
    });

    let response = client.new_room(request).await?;

    println!("RESPONSE={:?}", response);

    // new room
    // join room -> stream
    // room status -> draw first
    // listen stream and draw: should not be able to read first initmsg
    // get a roominfo: new player join in, if all 4 join in, display ready state
    // get a whoready: someone get ready
    // get a start: server start game, and client should rpc GameStatus to get cards
    // continue listen stream
    //
    // handle when someone exits
    //
    run_tui()?;
    Ok(())
}

fn run_tui() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
