use crate::{*, heart7_client::*};
use tonic::transport::Channel;
use tokio::sync::mpsc;
use tui::app::Action;
use std::net::Ipv4Addr;
use crate::tui::app::AppResult;

#[derive(Clone)]
pub struct Client {
    c: Heart7Client<Channel>,
    addr: String,
}

impl Client {
    pub fn connect_spawn(addr: &str, tx: &mpsc::Sender<Action>) {
        let txc = tx.clone();
        let addr = addr.to_string();
        tokio::spawn(async move {
            let (ip, port): (String, String) = match addr.find(':') {
                Some(i) => (addr[0..i].into(), addr[i+1..].into()),
                None => ("".into(), "".into())
            };
            txc.send(Action::ServerConnectResult(
                if ip.len() == 0 || port.len() == 0 {
                    Err("Invalid ip or port!".into())
                } else if !ip.parse::<Ipv4Addr>().is_ok() {
                    Err("Invalid ip address!".into())
                } else {
                    let url =format!("http://{}", &addr);
                    match Heart7Client::connect(url).await {
                        Ok(c) => Ok(Client{ c, addr }),
                        Err(e) => Err(e.to_string()),
                    }
                }
            )).await.expect("Send Action::ServerConnectResult to app");
        });
    }

    pub fn get_addr(&self) -> String {
        self.addr.clone()
    }

    pub async fn new_room(&mut self, name: String) -> AppResult<String> {
        let request = tonic::Request::new(PlayerInfo {
            name
        });

        Ok(self.c.new_room(request).await?.into_inner().roomid)
    }
}
