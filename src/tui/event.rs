use super::app::{AppResult, Action};
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

    pub fn run(&self, channel_buffer_size: usize,
               cancel: &CancellationToken,
               app_tx: mpsc::Sender<Action>,
    ) -> AppResult<()> {
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
                            Some(Event::Key(key)) => {
                                Self::handle_key_events(key, &app_tx).await;
                            },
                            Some(Event::Mouse(_)) => {}
                            Some(Event::Resize(x, y)) => {
                                app_tx.send(Action::Resize(x, y)).await
                                    .expect("Send Action::Resize to app");
                            }
                            Some(Event::Error) => {
                                panic!("Received Error from crossterm_event!");
                            }
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

    async fn handle_key_events(key: KeyEvent, tx: &mpsc::Sender<Action>) {
        match key.code {
            KeyCode::Enter => {
                tx.send(Action::Enter).await.expect("Send Action::Enter to app");
            }
            KeyCode::Esc => {
                tx.send(Action::Esc).await.expect("Send Action::Esc to app");
            }
            KeyCode::Char(c) => {
                if key.modifiers == KeyModifiers::CONTROL
                   && (c == 'c' || c == 'C') {
                    tx.send(Action::CtrlC).await.expect("Send Action::CtrlC to app");
                } else if key.modifiers == KeyModifiers::CONTROL
                   && (c == 'l' || c == 'L') {
                    tx.send(Action::Refresh).await.expect("Send Action::Refresh to app");
                } else {
                    tx.send(Action::Type(c)).await.expect("Send Action::Type to app");
                }
            }
            KeyCode::Left => {
                tx.send(Action::LeftArrow).await.expect("Send Action::LeftArrow to app");
            }
            KeyCode::Right => {
                tx.send(Action::LeftArrow).await.expect("Send Action::RightArrow to app");
            }
            KeyCode::Backspace => {
                tx.send(Action::Backspace).await.expect("Send Action::Backspace to app");
            }
            KeyCode::Delete => {
                tx.send(Action::Delete).await.expect("Send Action::Delete to app");
            }
            _ => {}
        }
    }
}
