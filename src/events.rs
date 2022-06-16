use std::{sync::Arc, time::Duration};

use async_tungstenite::tungstenite::Message;
use serde_json::{from_str, to_string};
use strife_types::ops::{op1::Op1, op10::Op10, op11::Op11};
use tokio::{sync::RwLock, time::sleep};
use tracing::{info, instrument};

use crate::event_manager::EventManager;

#[instrument(skip_all)]
pub(crate) async fn check_for_update(self_struct: Arc<RwLock<EventManager>>) {
    info!("Check For Update ran?");
    loop {
        info!(
            "{:#?}",
            EventManager::request_event(self_struct.clone(), "check_for_update").await
        );
        sleep(Duration::from_millis(500)).await;
    }
}

#[instrument(skip_all)]
pub(crate) async fn heartbeat_loop(self_struct: Arc<RwLock<EventManager>>, op10: Arc<Op10>) {
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
        EventManager::send(self_struct.clone(), Message::Text(heartbeat_data)).await;
        let msg: Op11 = from_str(
            EventManager::request_event(self_struct.clone(), "heartbeat_loop")
                .await
                .as_str(),
        )
        .unwrap();
        info!("HeartBeatLoop Recieved: {:#?}", msg);
        sleep(Duration::from_millis(op10.d.heartbeat_interval as u64)).await;
    }
}
