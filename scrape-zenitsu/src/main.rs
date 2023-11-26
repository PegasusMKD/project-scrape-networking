/**
*   This should be the UDP Server in which all calculations and logic for the game are done
*/
use std::io;

use tokio::net::UdpSocket;
use udp_server::game_server;
use udp_server::inbound_server;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut server = inbound_server::InboundServer::new(socket);

    game_server::GameServer::with_inbound(&server);

    println!("Started UDP server on port 8080.");

    loop {
        server.wait_incoming_messages().await?;
    }
}
