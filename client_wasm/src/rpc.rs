pub mod heart7_rpc {
    tonic::include_proto!("heart7_rpc");
}

pub use tonic::{Code, Request, Status};
pub use heart7_rpc::{*, heart7_client::*, game_msg::*, room_info::*};
use tonic_web_wasm_client::Client;
use web_sys::console;

fn build_client() -> Heart7Client<Client> {
    let base_url = "http://localhost:20007".to_string();
    let wasm_client = Client::new(base_url);

    Heart7Client::new(wasm_client)
}

pub async fn test_rpc() {
    let mut c = build_client();

    let request = Request::new(NewRoomReq {
        roomid: "123123".into()
    });

    let r = c.new_room(request).await.unwrap().into_inner();
    let msg = if r.success {
        format!("test_rpc gets {}", r.msg)
    } else {
        format!("test_rpc failed")
    };
    console::log_1(&msg.as_str().into());
}
