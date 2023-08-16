use super::app::AppResult;
use super::error::TuiError;
use crossterm::event::{
    Event as CrosstermEvent,
    EventStream,
    KeyEvent,
    KeyEventKind,
    MouseEvent,
};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Error,
}

#[derive(Debug)]
pub struct EventHandler {
    // tx: mpsc::Sender<Event>,
    // rx: mpsc::Receiver<Event>,
    // handler: JoinHandle<()>,
    cancel: CancellationToken,
}

impl EventHandler {
    pub fn new() -> Self {
        let cancel = CancellationToken::new();

        Self {
            cancel,
        }
    }

    pub fn run(&self, channel_buffer_size: usize) -> AppResult<()> {
        let (tx, mut rx) = mpsc::channel(channel_buffer_size);

        // spawn crossterm event poller task
        let cancel_clone = self.cancel.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            let mut reader = EventStream::new();
            loop {
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    _ = cancel_clone.cancelled() => {
                        break;
                    }
                    maybe_event = crossterm_event => {
                        match maybe_event {
                            Some(Ok(evt)) => {
                                Self::crossterm_event_handler(evt, &tx).await;
                            }
                            Some(Err(_)) => {
                                tx.send(Event::Error).await.expect(
                                    "Crossterm_event channel send failed"
                                );
                            }
                            None => {},
                        }
                    },
                }
            }
        });

        // spawn crossterm event handler task
        let cancel_clone = self.cancel.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    evt = rx.recv() => {
                        match evt {
                            None => panic!("Channel to crossterm_event closed!"),
                            Some(Event::Tick) => {},
                            Some(Event::Key(key)) => Self::handle_key_events(key).await,
                            Some(Event::Mouse(_)) => {}
                            Some(Event::Resize(_, _)) => {}
                            Some(Event::Error) => {}
                        }
                    }
                    _ = cancel_clone.cancelled() => {
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    pub fn stop(&mut self) {
        self.cancel.cancel();
    }

    async fn crossterm_event_handler(evt: CrosstermEvent, tx: &mpsc::Sender<Event>) {
        match evt {
            CrosstermEvent::Key(key) => {
                if key.kind == KeyEventKind::Release {
                    tx.send(Event::Key(key)).await
                        .expect("Crossterm_event channel send failed");
                }
            },
            CrosstermEvent::Resize(x, y) => {
                tx.send(Event::Resize(x, y)).await
                    .expect("Crossterm_event channel send failed");
            },
            _ => {},
        }
    }

    pub async fn handle_key_events(key: KeyEvent) {

    }
}

