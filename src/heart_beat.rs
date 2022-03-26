use std::{sync::Arc, thread::sleep, time::Duration};

use futures_util::{StreamExt, stream::{SplitSink, SplitStream}, lock::Mutex, SinkExt};
use serde_json::{from_str, to_string};
use async_tungstenite::{tokio::{connect_async, TokioAdapter}, tungstenite::Message, WebSocketStream, stream::Stream};
use tokio::net::TcpStream;
use tokio_native_tls::TlsStream;

use strife_types::{ops::{op10::Op10, op1::Op1, op11::Op11, op2::{Op2, Op2Data}}, properties::Properties, bot_gateway::BotGateway};

type StreamType = WebSocketStream<Stream<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TcpStream>>>>;

#[derive(Debug)]
pub struct Heartbeat {
    bot_gateway: Arc<BotGateway>,
    seq: Option<i32>,
    stream: Option<Arc<Mutex<StreamType>>>,
    session_id: Option<String>,
}

impl Heartbeat {
    pub fn new(bot_gateway: Arc<BotGateway>) -> Self {
        let y = Heartbeat {
            bot_gateway,
            seq: None,
            stream: None,
            session_id: None,
        };
        y
    }
    pub async fn run(&mut self, bot_token: String) {
        let url = format!("{}/?v=9&encoding=json", (*self.bot_gateway.clone()).url.as_str());

        self.stream = Some(Arc::new(Mutex::new(connect_async(&url)
            .await
            .expect("Failed to connect").0)));   

        let first_beat: Op10 = from_str(&format!("{}", &self.receive().await.to_string())).unwrap();
        println!("{:#?}", first_beat.clone());

        let first_beat = Arc::new(first_beat);

        self.heartbeat_loop(first_beat).await;

        let identity = to_string(&Op2 {
            op: 2,
            d: Some(Op2Data {
                token: bot_token,
                properities: Properties {
                    #[cfg(windows)]
                    os: "Windows".to_string(),
                    #[cfg(target_os = "macos")]
                    os: "Macos".to_string(),
                    #[cfg(target_os = "linux")]
                    os: "Linux".to_string(),
                    browser: "Strife".to_string(),
                    device: "Strife".to_string(),
                },
                compress: None,
                large_threshold: None,
                shards: None,
                intents: (1 << 8),
            }),
        }).unwrap();
        
        self.send(Message::text(identity)).await;
        let response = self.receive().await;
        println!("{}", response.to_string());
    }

    async fn heartbeat_loop(&self, op10: Arc<Op10>) {
        loop {
            let heartbeat_data = to_string(&Op1 {
                op: 1,
                d: self.seq,
                s: None,
            }).unwrap();

            println!("Sent: {:#?}", heartbeat_data);
            self.send(Message::Text(heartbeat_data)).await;
            let msg: Op11 = from_str(&self.receive().await.to_string()).unwrap();
            println!("Recieved: {:#?}", msg);
            sleep(Duration::from_millis(op10.d.heartbeat_interval as u64));
        }
    }
    async fn send(&self, message: Message) {
        self.stream.clone().unwrap().lock().await.send(message).await.unwrap();
    }
    async fn receive(&self) -> Message {
        self.stream.clone().unwrap().lock().await.next().await.unwrap().unwrap()
    }
}