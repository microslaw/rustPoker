use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;



#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("Server is running on 127.0.0.1:7878");

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Client connected!");

        tokio::spawn(async move {
            let mut buffer = [0; 512];
            match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    let received_message = String::from_utf8_lossy(&buffer[..n]);
                    println!("Received from client: {}", received_message);

                    let response = "Hello from server!";
                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                        eprintln!("Failed to send response: {}", e);
                    } else {
                        println!("Response sent to client.");
                    }
                }
                Err(e) => eprintln!("Failed to read from socket: {}", e),
                _ => println!("Client disconnected."),
            }
        });
    }
}
