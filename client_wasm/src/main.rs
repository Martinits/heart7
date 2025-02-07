mod event;
mod ui;
pub use event::*;
pub use ui::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
pub use tonic::{Code, Request, Status};
use tonic_web_wasm_client::Client;
pub use ::heart7_client::*;
use std::net::Ipv4Addr;
use async_channel::{bounded, Sender, Receiver};
use std::rc::Rc;
use std::cell::RefCell;

pub(crate) type JsResult<T> = Result<T, JsValue>;

pub const DEFAULT_IP: &str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 20007;

pub const DEFAULT_CHANNEL_SIZE: usize = 64;

pub fn spawn_tx_send(tx: Sender<ClientEvent>, payload: ClientEvent) {
    spawn_local(async move {
        tx.send(payload).await.unwrap();
    });
}

fn build_client(addr: String) -> Result<RpcClient, String> {
    let (ip, port): (String, String) = match addr.find(':') {
        Some(i) => (addr[0..i].into(), addr[i+1..].into()),
        None => ("".into(), "".into())
    };

    if ip.len() == 0 || port.len() == 0 {
        Err("Invalid ip or port!".into())
    } else if !ip.parse::<Ipv4Addr>().is_ok() {
        Err("Invalid ip address!".into())
    } else {
        let url = format!("http://{}", &addr);
        let web_client = Client::new(url);
        Ok(RpcClient::new(Heart7Client::new(web_client), addr))
    }
}

fn spawn_event_handler(tx: Sender<ClientEvent>, csm: CSMType) -> JsResult<()> {
    // canvas click event
    info!("Starting canvas click handler...");
    let txc = tx.clone();
    let listener = gloo::events::EventListener::new(&get_canvas(), "click", move |e| {
        let event = e.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
        let (left, top) = get_canvas_position();
        handle_click(
            event.client_x() as f64 - left,
            event.client_y() as f64 - top,
            txc.clone(),
            csm.borrow().get_client_state_brief(),
        ).unwrap_throw();
    });
    listener.forget();

    // hidden input input event
    info!("Starting hidden input input handler...");
    let txc = tx.clone();
    let listener = gloo::events::EventListener::new(&get_hidden_input(), "input", move |_| {
        let value = get_hidden_input().value();
        let txcc = txc.clone();
        spawn_tx_send(txcc, ClientEvent::ResetInput(value));
    });
    listener.forget();

    // hidden input blur event
    info!("Starting hidden input blur handler...");
    let txc = tx.clone();
    let listener = gloo::events::EventListener::new(&get_hidden_input(), "blur", move |_| {
        let value = get_hidden_input().value();
        let txcc = txc.clone();
        spawn_tx_send(txcc, ClientEvent::ResetInput(value));
    });
    listener.forget();

    Ok(())
}

struct StreamCancelToken;

type CSMType = Rc<RefCell<ClientStateManager>>;

struct ClientWasm {
    csm: CSMType,
    tx: Sender<ClientEvent>,
    rx: Receiver<ClientEvent>,
    stream_tx: Sender<StreamCancelToken>,
    stream_rx: Receiver<StreamCancelToken>,
    default_addr: String, // It's also the init value of the hidden_input
}

impl ClientWasm {
    pub fn new(default_addr: String) -> Self {
        let (tx, rx) = bounded(DEFAULT_CHANNEL_SIZE);
        let (stream_tx, stream_rx) = bounded(2);

        Self {
            csm: Rc::new(RefCell::new(ClientStateManager::new(default_addr.clone()))),
            tx,
            rx,
            stream_tx,
            stream_rx,
            default_addr,
        }
    }

    fn spawn_stream_listener(&mut self, mut gs: GameStream) {
        let txc = self.tx.clone();
        let cancel_rx = self.stream_rx.clone();
        info!("Spawning GameStream listener...");
        spawn_local(async move {
            txc.send(ClientEvent::StreamListenerSpawned).await
                .expect("Send Action::StreamListenerSpawned to client");
            loop {
                if let Ok(_) = cancel_rx.try_recv() {
                    break;
                }
                match gs.message().await {
                    Err(s) => panic!("GameStream error: {}", s),
                    Ok(None) => {
                        info!("GameStream closed! Stream listener exits!");
                        break;
                    }
                    Ok(Some(msg)) => txc.send(ClientEvent::StreamMsg(msg)).await
                        .expect("Send Action::StreamMsg to client"),
                }
            }
        });
    }

    fn spawn_rpc_client(&mut self, addr: String) {
        let txc = self.tx.clone();
        spawn_local(async move {
            let c = build_client(addr);
            txc.send(ClientEvent::ServerConnectResult(c))
                .await.expect("Send Action::ServerConnectResult to client");
        });
    }

    pub async fn run(&mut self) -> JsResult<()> {
        spawn_event_handler(self.tx.clone(), self.csm.clone())?;
        ui_init(self.default_addr.clone())?;

        // draw first anyway
        self.draw()?;
        loop {
            match self.rx.recv().await {
                Err(e) => {
                    warn!("channel to client closed: {}", e);
                    break;
                }
                Ok(e) => {
                    let reply = self.csm.borrow_mut().advance(e, should_block()).await;
                    if reply.full_exit {
                        // exit the "run" function only,
                        // the "exit" function will do the cancelling later.
                        break;
                    }
                    if reply.cancel_stream_listener {
                        self.cancel_stream_listener();
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

        Ok(())
    }

    fn draw(&mut self) -> JsResult<()> {
        Ok(draw(self.csm.borrow().get_client_state())?)
    }

    fn cancel_stream_listener(&self) {
        let _ = self.stream_tx.send(StreamCancelToken{});
    }

    pub fn exit(self) -> JsResult<()> {
        self.cancel_stream_listener();
        Ok(())
    }
}

fn main() {
    spawn_local(async move {
        let mut client = ClientWasm::new(format!("{}:{}", DEFAULT_IP, DEFAULT_PORT));

        info!("Heart7 Client Starts!");
        client.run().await.expect("Running ClientWasm");

        info!("Exiting...");
        client.exit().expect("Exiting ClientWasm");
    });
}
