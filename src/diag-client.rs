/* http client + runtime */
extern crate tokio;
extern crate hyper;
/* logging */
extern crate log;
extern crate env_logger;

use std::str::FromStr;
use hyper::{Request, Client, Body, Uri};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  std::env::set_var("RUST_LOG", "debug");
  env_logger::init();
  // start timer
  let start_time = std::time::Instant::now();
  // fire request
  let server_ip = String::from("127.0.0.1");
  let server_port = 3000;
  let request_url = format!("http://{}:{}/ping", server_ip, server_port);
  let client = Client::new();
  let request = Request::builder()
    .method("GET")
    .uri(Uri::from_str(&request_url).unwrap())
    .body(Body::empty())
    .unwrap();
  println!("requesting to diag-tunnel-server from diag-client {}", request_url);
  let response = client.request(request).await.unwrap();
  println!("got response from diag-tunnel-server {}", response.status());
  let response_body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
  println!("{:?}ms {:?}", start_time.elapsed().as_millis(), response_body);
  Ok(())
}
