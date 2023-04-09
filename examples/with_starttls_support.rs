use anyhow::{
    Result,
};
use async_trait::async_trait;
use tokio::io::{self, AsyncReadExt, AsyncRead};

use rust_smtp_server::backend::{Backend, Session, MailOptions};
use rust_smtp_server::server::Server;

struct MyBackend;

struct MySession;

impl Backend for MyBackend {
    type S = MySession;

    fn new_session(&self) -> Result<MySession> {
        Ok(MySession)
    }
}

#[async_trait]
impl Session for MySession {
    fn auth_plain(&mut self, _username: &str, _password: &str) -> Result<()> {
        Ok(())
    }
    
    async fn mail(&mut self, from: &str, _: &MailOptions) -> Result<()> {
        println!("mail from: {}", from);
        Ok(())
    }

    async fn rcpt(&mut self, to: &str) -> Result<()> {
        println!("rcpt to: {}", to);
        Ok(())
    }
    
    async fn data<R: AsyncRead + Send + Unpin>(&mut self, r: R) -> Result<()> {
        // print whole message
        let mut data = Vec::new();
        let mut reader = io::BufReader::new(r);
        reader.read_to_end(&mut data).await?;
        println!("data: {}", String::from_utf8_lossy(&data));

        Ok(())
    }

    fn reset(&mut self) {}

    fn logout(&mut self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let be = MyBackend;

    let mut s = Server::new(be);

    s.addr = "127.0.0.1:2525".to_string();
    s.domain = "localhost".to_string();
    s.read_timeout = std::time::Duration::from_secs(10);
    s.write_timeout = std::time::Duration::from_secs(10);
    s.max_message_bytes = 10 * 1024 * 1024;
    s.max_recipients = 50;
    s.max_line_length = 1000;
    s.allow_insecure_auth = true;

    println!("Starting server on {}", s.addr);
    match s.listen_and_serve().await {
        Ok(_) => println!("Server stopped"),
        Err(e) => println!("Server error: {}", e),
    }

    Ok(())
}