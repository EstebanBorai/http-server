use crate::config::Config;
use crate::file_explorer::FileExplorer;
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, StatusCode, Method};
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

    if self.must_log {
      println!("Listening on: http://{}", self.address.to_string());
    }

    if let Err(e) = server.await {
      eprintln!("Failed to initialize server: {}", e);
    }
  }

  async fn handle_requests(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let uri = req.uri();
    let method = req.method();

    match *method {
      Method::GET => Ok(Response::new("Hello, World".into())),
      _ => {
        let mut response = Response::new("Method Not Allowed".into());

        *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;

        Ok(response)
      }
    }
  }
}
