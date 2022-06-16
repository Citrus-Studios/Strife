use std::sync::Arc;

use async_tungstenite::tungstenite::Message;
use futures_util::SinkExt;
use serde_json::{from_str, to_string};
use strife_types::{
    ops::{
        op0::Op0,
        op10::Op10,
        op2::{Op2, Op2Data},
    },
    properties::Properties,
};
use tokio::sync::RwLock;
use tokio_stream::StreamExt;
use tracing::{info, instrument};

use crate::client::StreamType;

pub(crate) struct EventManager {
    pub(crate) stream: Option<Arc<RwLock<StreamType>>>,
    pub(crate) seq: i32,
}

impl EventManager {
    #[instrument(skip_all)]
    pub(crate) async fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            stream: None,
            seq: -1,
        }))
    }
    #[instrument(skip_all)]
    pub(crate) async fn send(self_struct: Arc<RwLock<Self>>, message: Message) {
        self_struct
            .write()
            .await
            .stream
            .clone()
            .unwrap()
            .write()
            .await
            .send(message)
            .await
            .unwrap();
    }
    #[instrument(skip_all)]
    pub(crate) async fn receive(self_struct: Arc<RwLock<Self>>) -> Message {
        self_struct
            .write()
            .await
            .stream
            .clone()
            .unwrap()
            .write()
            .await
            .next()
            .await
            .unwrap()
            .unwrap()
    }
    #[instrument(skip_all)]
    pub(crate) async fn initial_handshake(
        self_struct: Arc<RwLock<Self>>,
        bot_token: String,
    ) -> Arc<Op10> {
        let first_beat: Op10 = from_str(&format!(
            "{}",
            Self::receive(self_struct.clone()).await.to_string()
        ))
        .unwrap();
        info!("First Beat: {:#?}", first_beat.clone());

        let first_beat = Arc::new(first_beat);

        let identity = to_string(&Op2 {
            op: 2,
            d: Some(Op2Data {
                token: bot_token,
                properties: Properties {
                    #[cfg(windows)]
                    os: "windows".to_string(),
                    #[cfg(target_os = "macos")]
                    os: "macos".to_string(),
                    #[cfg(target_os = "linux")]
                    os: "linux".to_string(),
                    browser: "Strife".to_string(),
                    device: "Strife".to_string(),
                },
                compress: None,
                large_threshold: None,
                shards: None,
                intents: 1 << 9,
            }),
        })
        .unwrap();

        info!("Identity Sent: {}", identity);
        Self::send(self_struct.clone(), Message::text(identity)).await;
        let response = Self::receive(self_struct.clone()).await;

        if response.to_string() == "Disallowed intent(s)." {
            panic!("Disallowed Intents");
        }

        info!("Identity Recieved: {}", response.to_string());
        let response = from_str::<Op0>(response.to_string().as_str()).unwrap();
        info!("Identity Recieved: {:#?}", response);

        self_struct.clone().write().await.seq = response.s;
        return first_beat;
    }
}
