# Tokio Network Server
A simple network server built using the Tokio framework in Rust, showcasing its asynchronous capabilities and efficient handling of multiple connections.

## Requirements
- Rust 1.67 or later
- Tokio 1.24 or later
- Cargo

## Running the Server
1. Clone the repository

```bash
git clone git@github.com:CauaneAndrade/tokio-network-server.git
```

2. Build and run the server
```bash
cargo run
```

3. The server will be listening on `127.0.0.1:8000` by default. You can change the address and port by modifying the code.

## Usage
By default, any data sent to the server will be printed to the console. You can modify the code to handle the data as you wish.

Connect to the server using netcat or any other network tool:
```bash
netcat localhost 8000
```