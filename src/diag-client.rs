/* http client + runtime */
extern crate tokio;
extern crate hyper;
/* logging */
extern crate log;
extern crate env_logger;

use std::str::FromStr;
use hyper::{Request, Client, Body, Uri};
use log::{trace};
use std::error::Error;

type BoxedError = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  env_logger::init();
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
  let response = client.request(request).await.unwrap();
  // check response
  if response.status() != 200 {
    panic!("bad status: {}", response.status());
  }
  let response_body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
  println!("{:?}", response_body_bytes);
  Ok(())
}

