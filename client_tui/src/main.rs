mod event;
mod tui;
mod ui;

use event::TermEventHandler;
use std::env;
use std::panic;
use tokio_util::sync::CancellationToken;
use tokio::sync::mpsc;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use clap::Parser;
use anyhow::Result;
pub use ::heart7_client::*;
use std::net::Ipv4Addr;
use tui::*;

pub const DEFAULT_PORT: u16 = 20007;
pub const DEFAULT_CHANNEL_SIZE: usize = 64;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    #[clap(default_value_t=format!("127.0.0.1:{}", DEFAULT_PORT))]
    addr: String,
}

pub(crate) fn add_cancel_to_panic(cancel: CancellationToken) {
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        cancel.cancel();
        panic_hook(panic);
    }));
}

struct ClientTui {
    c: ClientStateManager,
    tui: Tui,
    tx: mpsc::Sender<ClientEvent>,
    rx: mpsc::Receiver<ClientEvent>,
    // client can cancel TermEventHandler
    // a panic from TermEventHandler should cancel client, too.
    te_cancel: CancellationToken,
    // a panic from stream listener should NOT cancel client.
    stream_cancel: CancellationToken,
}

impl ClientTui {
    pub fn new(default_addr: String) -> Result<Self> {
        let (tx, rx) = mpsc::channel(DEFAULT_CHANNEL_SIZE);

        let stream_cancel = CancellationToken::new();
        let te_cancel = CancellationToken::new();
        add_cancel_to_panic(te_cancel.clone());
        add_cancel_to_panic(stream_cancel.clone());

        Ok(Self {
            c: ClientStateManager::new(default_addr),
            tui: Tui::new()?,
            tx,
            rx,
            stream_cancel,
            te_cancel,
        })
    }

    fn spawn_stream_listener(&mut self, mut gs: GameStream) {
        let txc = self.tx.clone();

        if self.stream_cancel.is_cancelled() {
            self.stream_cancel = CancellationToken::new();
        }
        let scancel = self.stream_cancel.clone();

        info!("Spawning GameStream listener...");
        tokio::spawn(async move {
            txc.send(ClientEvent::StreamListenerSpawned).await
                .expect("Send Action::StreamListenerSpawned to client");
            loop {
                tokio::select!{
                    _ = scancel.cancelled() => {
                        info!("stream listener is cancelled");
                        break;
                    }
                    maybe_msg = gs.message() => {
                        match maybe_msg {
                            Err(s) => panic!("GameStream error: {}", s),
                            Ok(None) => {
                                info!("GameStream closed! Stream listener exits!");
                                break;
                            }
                            Ok(Some(msg)) => txc.send(ClientEvent::StreamMsg(msg)).await
                                .expect("Send Action::StreamMsg to client"),
                        }
                    }
                }
            }
        });
    }

    fn spawn_rpc_client(&mut self, addr: String) {
        let txc = self.tx.clone();
        tokio::spawn(async move {
            let (ip, port): (String, String) = match addr.find(':') {
                Some(i) => (addr[0..i].into(), addr[i+1..].into()),
                None => ("".into(), "".into())
            };
            txc.send(ClientEvent::ServerConnectResult(
                if ip.len() == 0 || port.len() == 0 {
                    Err("Invalid ip or port!".into())
                } else if !ip.parse::<Ipv4Addr>().is_ok() {
                    Err("Invalid ip address!".into())
                } else {
                    let url = format!("http://{}", &addr);
                    match Heart7Client::connect(url).await {
                        Ok(c) => RpcClient::new(c, addr).await.map_err(|e| format!("{}", e)),
                        Err(e) => Err(e.to_string()),
                    }
                }
            )).await.expect("Send Action::ServerConnectResult to client");
        });
    }

    pub async fn run(&mut self) -> Result<()> {
        info!("Starting terminal event handler...");
        TermEventHandler::spawn(DEFAULT_CHANNEL_SIZE, self.te_cancel.clone(), self.tx.clone())?;

        // draw first anyway
        self.draw()?;
        loop {
            tokio::select! {
                // a panic from TermEventHandler, stop client
                _ = self.te_cancel.cancelled() => {
                    break;
                }
                event = self.rx.recv() => {
                    match event {
                        None => panic!("Channel to client closed!"),
                        Some(e) => {
                            let reply = self.c.advance(e, self.tui.should_block()?).await;
                            if reply.full_exit {
                                // exit the "run" function only,
                                // the "exit" function will do the cancelling later.
                                break;
                            }
                            if reply.cancel_stream_listener {
                                self.stream_cancel.cancel();
                            }
                            if let Some(addr) = reply.spawn_rpc_client {
                                self.spawn_rpc_client(addr);
                            }
                            if let Some(gs) = reply.spawn_stream_listener {
                                self.spawn_stream_listener(gs);
                            }
                            if reply.need_redraw {
                                self.draw()?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        if self.tui.should_block()? {
            self.tui.draw_blocked()?;
        } else {
            self.tui.draw(self.c.get_client_state())?;
        }
        Ok(())
    }

    pub fn exit(mut self) -> Result<()> {
        self.te_cancel.cancel();
        self.stream_cancel.cancel();
        self.tui.exit()?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if let Ok(f) = env::var("LOGFILE") {
        let logfile = FileAppender::builder().build(f.clone())?;
        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder()
                       .appender("logfile")
                       .build(LevelFilter::Debug))?;
        log4rs::init_config(config)?;
        info!("Logging to {}", f);
    }

    info!("Heart7 Client Starts!");

    let mut client = ClientTui::new(args.addr)?;
    client.run().await?;

    info!("Exiting...");
    client.exit()?;

    Ok(())
}
