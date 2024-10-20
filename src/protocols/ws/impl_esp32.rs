use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use esp_idf_svc::http::server::EspHttpServer;
use crate::protocols::ProtocolError;
use crate::protocols::ws::{WebSocket, WebSocketClientListerApi, WebSocketServerApi};
use anyhow::Error;

pub struct Esp32SyncWebSocketProtocolApi {
    pub queue: Arc<Mutex<VecDeque<WebSocketSession>>>,
}

pub struct WebSocketSession {
    
}

impl Esp32SyncWebSocketProtocolApi {
    pub fn new(
        esp_server: &mut EspHttpServer
    ) -> Self {
        let queue = Arc::new(Mutex::new(VecDeque::new()));
        let queue_clone = queue.clone();
        esp_server.ws_handler("", move |ws| {
            let mut queue = queue_clone.lock().expect("Could not lock ws queue");
            queue.push_back(WebSocketSession{});
            return Ok(());
            Err(ProtocolError::new("".to_string()))
        }).expect("Failed to register web socket handler for EspHttpServer");

        Esp32SyncWebSocketProtocolApi {
            queue
        }
    }
}

impl WebSocket for Esp32SyncWebSocketProtocolApi {
    async fn send_async(&mut self, client_id: &str, message: &[u8]) -> Result<(), ProtocolError> {
        todo!()
    }

    async fn receive_async(&mut self, client_id: &str) -> Result<Vec<u8>, ProtocolError> {
        todo!()
    }

    async fn disconnect_async(&mut self, client_id: &str) -> Result<(), ProtocolError> {
        todo!()
    }
}

impl WebSocketClientListerApi for Esp32SyncWebSocketProtocolApi {
    async fn accept_async(&mut self) -> Result<String, ProtocolError> {
        todo!()
    }
}

impl WebSocketServerApi for Esp32SyncWebSocketProtocolApi {
    async fn connect_to_server_async(&mut self) -> Result<(), ProtocolError> {
        todo!()
    }
}