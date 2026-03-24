//! Network module
//!
//! TCP-based multiplayer server and client.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

/// TCP game server
pub struct GameServer {
    listener: Option<TcpListener>,
    port: u16,
    running: bool,
    next_client_id: u32,
}

impl GameServer {
    /// Create new server on port
    pub fn new(port: u16) -> Self {
        Self {
            listener: None,
            port,
            running: false,
            next_client_id: 0,
        }
    }

    /// Start listening for connections
    pub fn start(&mut self) -> Result<(), String> {
        let addr = format!("0.0.0.0:{}", self.port);
        self.listener = Some(TcpListener::bind(&addr).map_err(|e| e.to_string())?);
        self.running = true;
        tracing::info!("Server listening on {}", addr);
        Ok(())
    }

    /// Accept new connection
    pub fn accept(&mut self) -> Result<(u32, TcpStream), String> {
        if let Some(ref listener) = self.listener {
            match listener.accept() {
                Ok((stream, addr)) => {
                    let client_id = self.next_client_id;
                    self.next_client_id += 1;
                    tracing::info!("Client {} connected from {}", client_id, addr);
                    Ok((client_id, stream))
                }
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("Server not started".to_string())
        }
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Stop server
    pub fn stop(&mut self) {
        self.running = false;
        self.listener = None;
        tracing::info!("Server stopped");
    }
}

/// TCP game client
pub struct GameClient {
    stream: Option<TcpStream>,
    address: String,
    connected: bool,
}

impl GameClient {
    /// Create new client
    pub fn new(address: &str) -> Self {
        Self {
            stream: None,
            address: address.to_string(),
            connected: false,
        }
    }

    /// Connect to server
    pub fn connect(&mut self) -> Result<(), String> {
        let stream = TcpStream::connect(&self.address).map_err(|e| e.to_string())?;
        stream.set_nonblocking(true).map_err(|e| e.to_string())?;
        self.stream = Some(stream);
        self.connected = true;
        tracing::info!("Connected to server at {}", self.address);
        Ok(())
    }

    /// Send data to server
    pub fn send(&mut self, data: &[u8]) -> Result<(), String> {
        if let Some(ref mut stream) = self.stream {
            stream.write_all(data).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Not connected".to_string())
        }
    }

    /// Receive data from server (non-blocking)
    pub fn receive(&mut self, buffer: &mut [u8]) -> Result<Option<usize>, String> {
        if let Some(ref mut stream) = self.stream {
            match stream.read(buffer) {
                Ok(0) => {
                    self.connected = false;
                    tracing::info!("Disconnected from server");
                    Ok(None)
                }
                Ok(n) => Ok(Some(n)),
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("Not connected".to_string())
        }
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Disconnect from server
    pub fn disconnect(&mut self) {
        self.stream = None;
        self.connected = false;
        tracing::info!("Disconnected from server");
    }
}
