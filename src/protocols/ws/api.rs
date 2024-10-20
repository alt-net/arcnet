use crate::protocols::ProtocolError;

pub trait WebSocket {
    /// Sends a message to a connected client over WebSocket.
    async fn send_async(&mut self, client_id: &str, message: &[u8]) -> Result<(), ProtocolError>;

    /// Receives a message from a connected client.
    async fn receive_async(&mut self, client_id: &str) -> Result<Vec<u8>, ProtocolError>;

    /// Disconnects the WebSocket connection with a client.
    async fn disconnect_async(&mut self, client_id: &str) -> Result<(), ProtocolError>;
}

pub trait WebSocketClientListerApi : WebSocket {
    /// Accepts an incoming WebSocket connection from a client (Relic/Lumina).
    async fn accept_async(&mut self) -> Result<String, ProtocolError>;
}

pub trait WebSocketServerApi : WebSocket {
    /// Establishes a WebSocket connection to the DataHaven server.
    async fn connect_to_server_async(&mut self) -> Result<(), ProtocolError>;
}