extern crate tokio;
/* logging */
extern crate log;
extern crate env_logger;

use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  std::env::set_var("RUST_LOG", "debug");
  env_logger::init();
  loop {
    // 1. connect to vehicle
    println!("connecting to vehicle client at 127.0.0.1:3000");
    let mut vehicle_remote = TcpStream::connect(&"127.0.0.1:3000".to_string()).await?;
    let (mut vehicle_remote_recv, mut vehicle_remote_send) = vehicle_remote.split();
    println!("connected to vehicle client");
    // 2. connect to diag tunnel
    println!("connecting to diag tunnel");
    let diag_tunnel_external_ip = std::env::var("DIAG_TUNNEL_EXTERNAL_IP").unwrap();
    let diag_tunnel_address = format!("{}:5555", diag_tunnel_external_ip);
    println!("connecting to diag tunnel at {}", diag_tunnel_address);
    let mut diag_tunnel_remote = TcpStream::connect(diag_tunnel_address).await?;
    let (mut diag_tunnel_remote_recv, mut diag_tunnel_remote_send) = diag_tunnel_remote.split();
    println!("connected to diag tunnel");
    // 3. pipe
    println!("piping data");
    let diag_to_vehicle = async {
        io::copy(&mut diag_tunnel_remote_recv, &mut vehicle_remote_send).await?;
        vehicle_remote_send.shutdown().await
    };
    let vehicle_to_diag = async {
        io::copy(&mut vehicle_remote_recv, &mut diag_tunnel_remote_send).await?;
        diag_tunnel_remote_send.shutdown().await
    };
    tokio::try_join!(diag_to_vehicle, vehicle_to_diag)?;
    println!("finished piping?");
  }
}
