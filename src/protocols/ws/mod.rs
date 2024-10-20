mod api;
mod impl_esp32;

pub use self::api::*;
pub use self::impl_esp32::*;

use async_trait::async_trait;
use crate::protocols::{ProtocolError, ProtocolWorker};

pub struct WebSocketProtocolWorker<ClientListerApi, ServerApi> {
    client_lister_api: ClientListerApi,
    server_api: ServerApi,
}

impl<ClientListerApi, ServerApi> WebSocketProtocolWorker<ClientListerApi, ServerApi> {
    pub fn new(
        client_lister_api: ClientListerApi,
        server_api: ServerApi,
    ) -> Self {
        Self {
            client_lister_api,
            server_api,
        }
    }

    pub fn default() -> WebSocketProtocolWorker<ClientListerApi, ServerApi>
    where
        ClientListerApi: WebSocketClientListerApi + Send + Default,
        ServerApi: WebSocketServerApi + Send + Default,
    {
        Self {
            client_lister_api: Default::default(),
            server_api: Default::default(),
        }
    }
}

#[async_trait]
impl<ClientListerApi, ServerApi> ProtocolWorker for WebSocketProtocolWorker<ClientListerApi, ServerApi>
where
    ClientListerApi: WebSocketClientListerApi + Send,
    ServerApi: WebSocketServerApi + Send
{
    async fn run(&mut self) -> Result<(), ProtocolError> {
        Ok(())
    }
}