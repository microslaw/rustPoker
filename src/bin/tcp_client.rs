use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878").await?;
    println!("Connected to the server!");

    let message = "Hello from client!";
    stream.write_all(message.as_bytes()).await?;
    println!("Message sent to server.");

    let mut buffer = [0; 512];
    match stream.read(&mut buffer).await {
        Ok(n) if n > 0 => {
            let response = String::from_utf8_lossy(&buffer[..n]);
            println!("Received from server: {}", response);
        }
        Err(e) => eprintln!("Failed to read from server: {}", e),
        _ => println!("Server closed the connection."),
    }

    Ok(())
}
