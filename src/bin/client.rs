use heart7::{*, heart7_client::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    Ok(())
}
