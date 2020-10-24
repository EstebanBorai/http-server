mod cli;
mod config;
mod file_explorer;
mod handler;
mod server;

#[tokio::main]
async fn main() {
    let cli_app = cli::make_app();
    let conf = config::Config::from(cli_app);
    let server = server::HttpServer::from(conf);

    server.serve().await;
}
