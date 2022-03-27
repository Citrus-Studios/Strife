use std::{sync::{Arc}, thread::sleep, time::Duration};

use futures_util::{StreamExt, SinkExt};
use serde_json::{from_str, to_string};
use async_tungstenite::{tokio::{connect_async, TokioAdapter}, tungstenite::Message, WebSocketStream, stream::Stream};
use tokio::{net::TcpStream, sync::RwLock};
use tokio_native_tls::TlsStream;

use strife_types::{ops::{op10::Op10, op1::Op1, op11::Op11, op2::{Op2, Op2Data}, op0::Op0}, properties::Properties, bot_gateway::BotGateway};

type StreamType = WebSocketStream<Stream<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TcpStream>>>>;

#[derive(Debug)]
pub struct Heartbeat {
    bot_gateway: Arc<BotGateway>,
    seq: i32,
    stream: Option<Arc<RwLock<StreamType>>>,
    session_id: Option<String>,
}

impl Heartbeat {
    pub fn new(bot_gateway: Arc<BotGateway>) -> Self {
        let y = Heartbeat {
            bot_gateway,
            seq: -1,
            stream: None,
            session_id: None,
        };
        y
    }
    pub async fn run(self, bot_token: String) {
        let arc_self = Arc::new(RwLock::new(self));

        let url = format!("{}/?v=9&encoding=json", (arc_self.clone().read().await.bot_gateway.clone()).url.as_str());

        arc_self.clone().write().await.stream = Some(Arc::new(RwLock::new(connect_async(&url)
            .await
            .expect("Failed to connect").0)));   

        

        let first_beat: Op10 = from_str(&format!("{}",Self::receive(arc_self.clone()).await.to_string())).unwrap();
        println!("First Beat: {:#?}", first_beat.clone());

        let first_beat = Arc::new(first_beat);

        let fut = Self::heartbeat_loop(arc_self.clone(), first_beat);

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
                intents: 0,
            }),
        }).unwrap();
        
        println!("Identity Sent: {}", identity);
        Self::send(arc_self.clone(), Message::text(identity)).await;
        let response = Self::receive(arc_self.clone()).await;

        if response.to_string() == "Disallowed intent(s)." {
            panic!("Disallowed Intents");
        }

        println!("Identity Recieved: {}", response.to_string());
        let response = from_str::<Op0>(response.to_string().as_str()).unwrap();
        println!("Identity Recieved: {:#?}", response);

        arc_self.clone().write().await.seq = response.s;

        fut.await;
    }

    async fn heartbeat_loop(self_struct: Arc<RwLock<Self>>, op10: Arc<Op10>) {
        loop {
            let x = self_struct.clone().read().await.seq;
            let heartbeat_data = to_string(&Op1 {
                op: 1,
                d: None,
                s: match x {
                    -1 => None,
                    _ => Some(x)
                },
            }).unwrap();

            println!("HeartBeatLoop Sent: {}", heartbeat_data);
            Self::send(self_struct.clone(), Message::Text(heartbeat_data)).await;
            let msg: Op11 = from_str(Self::receive(self_struct.clone()).await.to_string().as_str()).unwrap();
            println!("HeartBeatLoop Recieved: {:#?}", msg);
            sleep(Duration::from_millis(op10.d.heartbeat_interval as u64));
        }
    }
    async fn send(self_struct: Arc<RwLock<Self>>, message: Message) {
        self_struct.write().await.stream.clone().unwrap().write().await.send(message).await.unwrap();
    }
    async fn receive(self_struct: Arc<RwLock<Self>>) -> Message {
        self_struct.write().await.stream.clone().unwrap().write().await.next().await.unwrap().unwrap()
    }
}