use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind a TCP listener to the address "127.0.0.1:8000"
    let listener = TcpListener::bind("127.0.0.1:8000").await?;

    // Print the local address that the listener is bound to
    println!("Listening on {}", listener.local_addr()?);

    // Wait for incoming connections and handle them in a separate task
    while let Ok((mut socket, _)) = listener.accept().await {
        // Create a new task to handle the connection
        tokio::spawn(async move {
            // Split the socket into a reader and a writer
            let (mut reader, _writer) = socket.split();

            // Copy data from the socket to stdout
            tokio::io::copy(&mut reader, &mut tokio::io::stdout())
                .await
                .expect("Failed to copy data");
        });
    }

    Ok(())
}
