use heart7::{*, heart7_client::*};

use tui::app::{App, AppResult};
use tui::event::EventHandler;
use tui::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {

    let mut app = App::new();

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    tui.init()?;

    let event = EventHandler::new();
    event.run(64)?;

    // spawn state manager + render task
    // main task: rpc client


    while app.running {
        tui.draw(&mut app)?;
        // match tui.events.next().await? {
        //     Event::Tick => app.tick(),
        //     Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
        //     Event::Mouse(_) => {}
        //     Event::Resize(_, _) => {}
        //     Event::Error => {}
        // }
    }

    tui.exit()?;

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
    Ok(())
}
