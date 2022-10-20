use common::frame::FrameJson;
use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};

use wasm_bindgen_futures::spawn_local;
use yew_agent::Dispatched;

use crate::event_bus::{EventBus, Request};

pub struct WebsocketService {
    pub tx: Sender<FrameJson>,
}

pub enum WebsocketMsg {
    Send(FrameJson),
}

impl WebsocketService {
    pub fn new() -> Self {
        // create websocket connection
        let ws = WebSocket::open("ws://127.0.0.1:8080/api/ws").unwrap();

        // split the websocket into a sender (write) and receiver (read)
        let (mut write, mut read) = ws.split();

        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<FrameJson>(1000);
        let mut event_bus = EventBus::dispatcher();

        spawn_local(async move {
            while let Some(s) = in_rx.next().await {
                log::debug!("got event from channel! {:#?}", s);

                write
                    .send(Message::Text(serde_json::to_string(&s).unwrap()))
                    .await
                    .unwrap();
            }
        });

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(data)) => {
                        log::debug!("from websocket: {}", data);
                    }
                    Ok(Message::Bytes(b)) => {
                        let decoded = std::str::from_utf8(&b);
                        if let Ok(val) = decoded {
                            log::debug!("from websocket: {}", val);
                            event_bus.send(Request::EventBusMsg(val));
                        }
                    }
                    Err(e) => {
                        log::error!("ws: {:?}", e)
                    }
                }
            }
            log::debug!("WebSocket Closed");
        });

        Self { tx: in_tx }
    }
}
