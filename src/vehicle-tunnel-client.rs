extern crate tokio;
/* logging */
extern crate log;
extern crate env_logger;

use tokio::io;
use tokio::net::{TcpStream};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  std::env::set_var("RUST_LOG", "debug");
  env_logger::init();
  loop {
    // 1. connect to vehicle
    let vehicle_server_ip = "127.0.0.1";
    let vehicle_server_port = 3000;
    println!("connecting to vehicle-server at {}:{}", vehicle_server_ip, vehicle_server_port);
    let mut vehicle_remote = TcpStream::connect(&"127.0.0.1:3000".to_string()).await?;
    vehicle_remote.set_nodelay(true)?;
    let (mut vehicle_remote_recv, mut vehicle_remote_send) = vehicle_remote.split();
    println!("connected to vehicle-server");
    // 2. connect to diag tunnel
    let diag_tunnel_external_ip = std::env::var("DIAG_TUNNEL_EXTERNAL_IP").unwrap();
    let diag_tunnel_address = format!("{}:5555", diag_tunnel_external_ip);
    println!("connecting to diag-tunnel-server at {}", diag_tunnel_address);
    let mut diag_tunnel_remote = TcpStream::connect(diag_tunnel_address).await?;
    diag_tunnel_remote.set_nodelay(true)?;
    let (mut diag_tunnel_remote_recv, mut diag_tunnel_remote_send) = diag_tunnel_remote.split();
    println!("connected to diag-tunnel-server");
    // 3. pipe
    println!("piping data");
    tokio::select! {
      _ = io::copy(&mut diag_tunnel_remote_recv, &mut vehicle_remote_send) => {
        println!("diag_tunnel_remote_recv -> vehicle_remote_send finished");
      }
      _ = io::copy(&mut vehicle_remote_recv, &mut diag_tunnel_remote_send) => {
        println!("vehicle_remote_recv -> diag_tunnel_remote_send finished");
      }
    }
    // TODO: which to shutdown?
    println!("finished piping?");
  }
  Ok(())
}
