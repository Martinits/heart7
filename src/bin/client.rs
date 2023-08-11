use heart7_package::heart7_client::Heart7Client;
use heart7_package::PlayerInfo;

pub mod heart7_package {
    tonic::include_proto!("heart7_rpc");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Heart7Client::connect("http://localhost:20007").await?;

    let request = tonic::Request::new(PlayerInfo {
        name: "Martinits".into(),
    });

    let response = client.new_room(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
