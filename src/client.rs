use std::sync::Arc;

use async_tungstenite::{
    stream::Stream,
    tokio::{connect_async, TokioAdapter},
    WebSocketStream,
};
use tokio::{net::TcpStream, sync::RwLock};
use tokio_native_tls::TlsStream;

use strife_types::bot_gateway::BotGateway;
use tracing::instrument;

use crate::event_manager::EventManager;

pub type StreamType =
    WebSocketStream<Stream<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TcpStream>>>>;

pub struct Client {
    bot_gateway: Arc<BotGateway>,
    stream: Option<Arc<RwLock<StreamType>>>,
}

impl Client {
    #[instrument(skip_all)]
    pub fn new(bot_gateway: Arc<BotGateway>) -> Self {
        Self {
            bot_gateway,
            stream: None,
        }
    }
    #[instrument(skip_all)]
    pub(crate) async fn run(self, bot_token: String) {
        let self_struct = Arc::new(RwLock::new(self));

        let url = format!(
            "{}/?v=9&encoding=json",
            (self_struct.clone().read().await.bot_gateway.clone())
                .url
                .as_str()
        );

        self_struct.clone().write().await.stream = Some(Arc::new(RwLock::new(
            connect_async(&url).await.expect("Failed to connect").0,
        )));

        let first_beat =
            EventManager::initial_handshake(EventManager::new().await, bot_token).await;
    }
}
