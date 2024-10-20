use async_trait::async_trait;
use esp_idf_svc::http::server::EspHttpServer;
use self::ws::Esp32SyncWebSocketProtocolApi;

pub mod ws;

#[derive(Debug)]
pub struct ProtocolError {
    message: String,
}

impl ProtocolError {
    pub fn new(message: String) -> Self {
        Self {
            message
        }
    }
}

#[async_trait]
pub trait ProtocolWorker {
    async fn run(&mut self) -> Result<(), ProtocolError>;
}

pub enum ProtocolType {
    WebSocket,
}

pub struct ProtocolManager {
    protocol_entries: Vec<Box<dyn ProtocolWorker + Send>>,
}

fn default_ws(esp_server: &mut EspHttpServer) -> Box<dyn ProtocolWorker + Send> {
    Box::new(ws::WebSocketProtocolWorker::new(
        Esp32SyncWebSocketProtocolApi::new(esp_server),
        Esp32SyncWebSocketProtocolApi::new(esp_server),
    )) as Box<dyn ProtocolWorker + Send>
}

impl ProtocolManager {
    pub fn new(
        protocols: &[ProtocolType],
        esp_server: &mut EspHttpServer,
    ) -> Self {
        let protocol_entries: Vec<Box<dyn ProtocolWorker + Send>> = protocols.iter().map(|protocol| match protocol {
            ProtocolType::WebSocket => default_ws(esp_server),
        }).collect();
    
        Self {
            protocol_entries,
        }
    }

    // pub async fn run(&mut self) -> Result<(), ProtocolError> {
    //     let mut tasks = vec![];
    // 
    //     for protocol_handler in &mut self.protocol_handlers {
    //         let mut local_executor = LocalPool::new();
    //         let spawner = local_executor.spawner();
    //         
    //         let task = task::spawn(async move {
    //             loop {
    //                 // Handle protocol communication, e.g., receive messages and process
    //                 let result = protocol_handler.receive_message().await;
    //                 match result {
    //                     Ok(Some(message)) => {
    //                         // Process message...
    //                     }
    //                     _ => break, // Handle errors or disconnection
    //                 }
    //             }
    //         });
    //         tasks.push(task);
    //     }
    // 
    //     // Await all tasks to complete
    //     for t in tasks {
    //         t.await?;
    //     }
    // 
    //     Ok(())
    // }
}
