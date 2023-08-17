use super::app::AppResult;
use crate::*;
use crossterm::event::{
    Event as CrosstermEvent,
    EventStream,
    KeyEvent,
    KeyEventKind,
    KeyCode,
    KeyModifiers,
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
pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, channel_buffer_size: usize, cancel: &CancellationToken) -> AppResult<()> {
        let (tx, mut rx) = mpsc::channel(channel_buffer_size);

        // spawn crossterm event poller task
        let cancel_clone = cancel.clone();
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
        let cancel_clone = cancel.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    evt = rx.recv() => {
                        match evt {
                            None => panic!("Channel to crossterm_event closed!"),
                            Some(Event::Tick) => {},
                            Some(Event::Key(key)) => Self::handle_key_events(key, &cancel_clone).await,
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

    async fn crossterm_event_handler(evt: CrosstermEvent, tx: &mpsc::Sender<Event>) {
        match evt {
            CrosstermEvent::Key(key) => {
                if key.kind == KeyEventKind::Press {
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

    async fn handle_key_events(key: KeyEvent, cancel: &CancellationToken) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {}
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key.modifiers == KeyModifiers::CONTROL {
                    cancel.cancel();
                }
            }
            KeyCode::Right => {}
            KeyCode::Left => {}
            _ => {}
        }
    }
}
