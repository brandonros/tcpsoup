#[macro_use]
extern crate tokio;

use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  loop {
    // 1. connect to diag proxy
    println!("connecting to diag proxy");
    let diag_proxy_address = std::env::var("DIAG_PROXY_ADDRESS").unwrap();
    let mut diag_proxy_remote = TcpStream::connect(diag_proxy_address).await?;
    let (mut diag_proxy_remote_recv, mut diag_proxy_remote_send) = diag_proxy_remote.split();
    println!("connected to diag proxy");
    // 2. connect to vehicle
    println!("connecting to vehicle client");
    let mut vehicle_remote = TcpStream::connect(&"127.0.0.1:3000".to_string()).await?;
    let (mut vehicle_remote_recv, mut vehicle_remote_send) = vehicle_remote.split();
    println!("connecyed to vehicle client");
    // 3. pipe
    println!("piping data");
    let diag_to_vehicle = async {
        io::copy(&mut diag_proxy_remote_recv, &mut vehicle_remote_send).await?;
        vehicle_remote_send.shutdown().await
    };
    let vehicle_to_diag = async {
        io::copy(&mut vehicle_remote_recv, &mut diag_proxy_remote_send).await?;
        diag_proxy_remote_send.shutdown().await
    };
    tokio::try_join!(diag_to_vehicle, vehicle_to_diag)?;
    println!("finished piping?");
  }
  Ok(())
}
