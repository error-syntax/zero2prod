use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub mod app;

pub async fn run() -> anyhow::Result<axum::serve::Serve<TcpListener, Router, Router>> {
  let app = app::app();

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  let listener = TcpListener::bind(addr).await?;

  println!("Starting server...");
  println!("Listening on {}", addr);

  Ok(axum::serve(listener, app))
}
