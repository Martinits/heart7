use crate::client::ClientEvent;
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
use anyhow::Result;

#[derive(Clone, Copy, Debug)]
pub enum TermEvent {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Error,
}

#[derive(Debug)]
pub struct TermEventHandler;

impl TermEventHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, channel_buffer_size: usize,
               cancel: &CancellationToken,
               client_tx: mpsc::Sender<ClientEvent>,
    ) -> Result<()> {
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
                                tx.send(TermEvent::Error).await.expect(
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
                            Some(TermEvent::Tick) => {},
                            Some(TermEvent::Key(key)) => {
                                Self::handle_key_events(key, &client_tx).await;
                            },
                            Some(TermEvent::Mouse(_)) => {}
                            Some(TermEvent::Resize(x, y)) => {
                                client_tx.send(ClientEvent::Resize(x, y)).await
                                    .expect("Send Action::Resize to client");
                            }
                            Some(TermEvent::Error) => {
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

    async fn crossterm_event_handler(evt: CrosstermEvent, tx: &mpsc::Sender<TermEvent>) {
        match evt {
            CrosstermEvent::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    tx.send(TermEvent::Key(key)).await
                        .expect("Crossterm_event channel send failed");
                }
            },
            CrosstermEvent::Resize(x, y) => {
                tx.send(TermEvent::Resize(x, y)).await
                    .expect("Crossterm_event channel send failed");
            },
            _ => {},
        }
    }

    async fn handle_key_events(key: KeyEvent, tx: &mpsc::Sender<ClientEvent>) {
        match key.code {
            KeyCode::Enter => {
                tx.send(ClientEvent::Enter).await.expect("Send Action::Enter to client");
            }
            KeyCode::Esc => {
                tx.send(ClientEvent::Esc).await.expect("Send Action::Esc to client");
            }
            KeyCode::Char(c) => {
                if key.modifiers == KeyModifiers::CONTROL
                   && (c == 'c' || c == 'C') {
                    tx.send(ClientEvent::CtrlC).await.expect("Send Action::CtrlC to client");
                } else if key.modifiers == KeyModifiers::CONTROL
                   && (c == 'l' || c == 'L') {
                    tx.send(ClientEvent::Refresh).await.expect("Send Action::Refresh to client");
                } else {
                    tx.send(ClientEvent::Type(c)).await.expect("Send Action::Type to client");
                }
            }
            KeyCode::Left => {
                tx.send(ClientEvent::LeftArrow).await.expect("Send Action::LeftArrow to client");
            }
            KeyCode::Right => {
                tx.send(ClientEvent::RightArrow).await.expect("Send Action::RightArrow to client");
            }
            KeyCode::Up => {
                tx.send(ClientEvent::UpArrow).await.expect("Send Action::UpArrow to client");
            }
            KeyCode::Down => {
                tx.send(ClientEvent::DownArrow).await.expect("Send Action::DownArrow to client");
            }
            KeyCode::Backspace => {
                tx.send(ClientEvent::Backspace).await.expect("Send Action::Backspace to client");
            }
            KeyCode::Delete => {
                tx.send(ClientEvent::Delete).await.expect("Send Action::Delete to client");
            }
            _ => {}
        }
    }
}
