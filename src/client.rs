use std::{collections::HashMap, sync::Arc, thread::sleep, time::Duration};

use async_tungstenite::{
    stream::Stream,
    tokio::{connect_async, TokioAdapter},
    tungstenite::Message,
    WebSocketStream,
};
use futures_util::{SinkExt, StreamExt};
use serde_json::{from_str, to_string};
use tokio::{net::TcpStream, spawn, sync::RwLock};
use tokio_native_tls::TlsStream;

use strife_types::{
    bot_gateway::BotGateway,
    ops::{
        op0::Op0,
        op1::Op1,
        op10::Op10,
        op11::Op11,
        op2::{Op2, Op2Data},
    },
    properties::Properties,
};
use tracing::{info, instrument};

type StreamType =
    WebSocketStream<Stream<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TcpStream>>>>;

pub struct Client {
    bot_gateway: Arc<BotGateway>,
    seq: i32,
    events: HashMap<String, Message>,
    stream: Option<Arc<RwLock<StreamType>>>,
    session_id: Option<String>,
}

impl Client {
    #[instrument(skip_all)]
    pub fn new(bot_gateway: Arc<BotGateway>) -> Self {
        Self {
            bot_gateway,
            seq: -1,
            events: HashMap::new(),
            stream: None,
            session_id: None,
        }
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

        let first_beat = Self::initial_handshake(self_struct.clone(), bot_token).await;

        let heart_beat_clone = self_struct.clone();
        let handle1 = spawn(async move {
            Self::heartbeat_loop(heart_beat_clone, first_beat).await;
        });
        let check_for_update_clone = self_struct.clone();
        let handle2 = spawn(async move {
            Self::check_for_update(check_for_update_clone).await;
        });
        handle1.await.unwrap();
        handle2.await.unwrap();
    }
    #[instrument(skip_all)]
    async fn check_for_update(self_struct: Arc<RwLock<Self>>) {
        info!("Check For Update ran?");
        loop {
            info!(
                "{:#?}",
                Self::receive(self_struct.clone()).await.to_string()
            );
            sleep(Duration::from_secs(1))
        }
    }
    #[instrument(skip_all)]
    async fn heartbeat_loop(self_struct: Arc<RwLock<Self>>, op10: Arc<Op10>) {
        loop {
            let x = self_struct.clone().read().await.seq;
            let heartbeat_data = to_string(&Op1 {
                op: 1,
                d: None,
                s: match x {
                    -1 => None,
                    _ => Some(x),
                },
            })
            .unwrap();

            info!("HeartBeatLoop Sent: {}", heartbeat_data);
            Self::send(self_struct.clone(), Message::Text(heartbeat_data)).await;
            let msg: Op11 = from_str(
                Self::receive(self_struct.clone())
                    .await
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            info!("HeartBeatLoop Recieved: {:#?}", msg);
            sleep(Duration::from_millis(op10.d.heartbeat_interval as u64));
        }
    }
    #[instrument(skip_all)]
    async fn send(self_struct: Arc<RwLock<Self>>, message: Message) {
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
    async fn receive(self_struct: Arc<RwLock<Self>>) -> Message {
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
}
