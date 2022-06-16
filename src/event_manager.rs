use std::{collections::HashMap, sync::Arc, time::Duration};

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
use tokio::{spawn, sync::RwLock, time::sleep};
use tokio_stream::StreamExt;
use tracing::{info, instrument};

use crate::{
    client::StreamType,
    events::{check_for_update, heartbeat_loop},
};

pub(crate) struct EventManager {
    pub(crate) stream: Arc<RwLock<StreamType>>,
    pub(crate) seq: i32,
    pub(crate) requests: HashMap<String, (String, bool)>,
}

impl EventManager {
    #[instrument(skip_all)]
    pub(crate) async fn new(stream: Arc<RwLock<StreamType>>) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            stream,
            seq: -1,
            requests: HashMap::new(),
        }))
    }
    #[instrument(skip_all)]
    pub(crate) async fn run(self_struct: Arc<RwLock<Self>>, bot_token: String) {
        let first_beat = Self::initial_handshake(self_struct.clone(), bot_token.to_string()).await;

        let mut locked = self_struct.write().await;
        locked
            .requests
            .insert(String::from("heartbeat_loop"), (String::from(""), false));
        locked
            .requests
            .insert(String::from("check_for_update"), (String::from(""), false));

        let heart_beat_clone = self_struct.clone();
        let handle1 = spawn(async move {
            heartbeat_loop(heart_beat_clone, first_beat).await;
        });
        let check_for_update_clone = self_struct.clone();
        let handle2 = spawn(async move {
            check_for_update(check_for_update_clone).await;
        });
        let event_loop_clone = self_struct.clone();
        let handle3 = spawn(async move {
            Self::event_loop(event_loop_clone).await;
        });
        handle1.await.unwrap();
        handle2.await.unwrap();
        handle3.await.unwrap();
    }
    pub(crate) async fn event_loop(self_struct: Arc<RwLock<Self>>) {
        info!("Event Loop Started");

        loop {
            let event = EventManager::receive(self_struct.clone()).await;
            info!("Received Event: {:#?}", event);
            sleep(Duration::from_secs(1)).await;
        }
    }
    #[instrument(skip_all)]
    pub(crate) async fn request_event(self_struct: Arc<RwLock<Self>>, value: &str) -> String {
        while self_struct
            .clone()
            .read()
            .await
            .requests
            .get(&value.to_string())
            .unwrap()
            .1
            == false
        {}
        return self_struct
            .clone()
            .read()
            .await
            .requests
            .get(&value.to_string())
            .unwrap()
            .0
            .clone();
    }
    #[instrument(skip_all)]
    pub(crate) async fn send(self_struct: Arc<RwLock<Self>>, message: Message) {
        self_struct
            .read()
            .await
            .stream
            .clone()
            .write()
            .await
            .send(message)
            .await
            .unwrap();
    }
    #[instrument(skip_all)]
    pub(crate) async fn receive(self_struct: Arc<RwLock<Self>>) -> Message {
        self_struct
            .read()
            .await
            .stream
            .clone()
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
        info!("First Beat");

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

        info!("Identity Sent");
        Self::send(self_struct.clone(), Message::text(identity)).await;
        let response = Self::receive(self_struct.clone()).await;

        if response.to_string() == "Disallowed intent(s)." {
            panic!("Disallowed Intents");
        }

        info!("Identity Recieved");
        let response = from_str::<Op0>(response.to_string().as_str()).unwrap();
        info!("Identity Recieved");

        self_struct.clone().write().await.seq = response.s;
        return first_beat;
    }
}
