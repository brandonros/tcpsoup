extern crate tokio;
/* logging */
extern crate log;
extern crate env_logger;

use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  std::env::set_var("RUST_LOG", "debug");
  env_logger::init();
  let vehicle_listener = TcpListener::bind(&"0.0.0.0:5555".to_string()).await?;
  let diag_listener = TcpListener::bind(&"127.0.0.1:3000".to_string()).await?;
  println!("bound diag-tunnel-server to 0.0.0.0:5555");
  println!("bound diag-client-listener to 127.0.0.1:3000");
  println!("waiting for vehicle tunnel to connect from external internet");
  let (mut vehicle_client, _vehicle_client_addr) = vehicle_listener.accept().await?;
  let (mut vehicle_client_recv, mut vehicle_client_send) = vehicle_client.split();
  println!("vehicle tunnel connected");
  loop {
      println!("waiting for incoming diag request connections");
      let (mut diag_client, _diag_client_addr) = diag_listener.accept().await?;
      let (mut diag_client_recv, mut diag_client_send) = diag_client.split();
      println!("got diag request connection");
      println!("piping data");
      let diag_to_vehicle = async {
          io::copy(&mut diag_client_recv, &mut vehicle_client_send).await
      };
      let vehicle_to_diag = async {
          io::copy(&mut vehicle_client_recv, &mut diag_client_send).await
      };
      tokio::try_join!(diag_to_vehicle, vehicle_to_diag)?;
      diag_client_send.shutdown().await?;
      // TODO: which to shutdown?
      println!("diag connection finished?");
  }
}
