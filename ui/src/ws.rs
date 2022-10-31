use common::frame::FrameJson;
use futures::{channel::mpsc::Sender, SinkExt, StreamExt};

use gloo_net::websocket::futures::WebSocket;
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
                    Ok(m) => {
                        let frame = match m {
                            Message::Text(s) => Ok(s),
                            Message::Bytes(b) => std::str::from_utf8(&b).map(|s| s.to_string()),
                        }
                        .map(|s| serde_json::from_str::<FrameJson>(&s));

                        match frame {
                            Ok(Ok(f)) => {
                                log::debug!("got event from websocket! {:#?}", f);
                                event_bus.send(Request::EventBusMsg(f));
                            }
                            _ => log::error!("failed to parse frame"),
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
