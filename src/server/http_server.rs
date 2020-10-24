use crate::config::Config;
use crate::file_explorer::FileExplorer;
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::server::Builder;
use hyper::server::conn::AddrIncoming;
use hyper::service::{make_service_fn, service_fn};

pub struct HttpServer {
  pub server: Builder<AddrIncoming>,
  pub address: SocketAddr,
  pub must_log: bool,
  pub file_explorer: FileExplorer,
}

impl From<Config> for HttpServer {
  fn from(conf: Config) -> Self {
    let address = conf.socket_address;
    let file_explorer = FileExplorer::new(conf.root_dir);
    let server = Server::bind(&address);

    Self {
      server,
      address,
      must_log: !conf.silent,
      file_explorer
    }
  }
}

impl HttpServer {
  pub async fn serve(self) {
    let main_service = make_service_fn(|_conn| async {
      Ok::<_, Infallible>(service_fn(Self::handle_requests))
    });

    let server = self.server.serve(main_service);

    if let Err(e) = server.await {
      eprintln!("Failed to initialize server: {}", e);
    }
  }

  async fn handle_requests(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
  }
}
