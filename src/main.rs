mod packets;
mod server;
mod utility;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = server::Server::new();
    server.run("0.0.0.0:25565").await?;
    Ok(())
}
