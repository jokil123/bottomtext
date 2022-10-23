use common::frame::FrameJson;
use futures::SinkExt;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::components::frame::Frame;
use crate::components::frame_input::FrameInput;
use crate::model::FrameModel;
use crate::{event_bus::EventBus, ws::WebsocketService};

pub enum Msg {
    RecieveMsg(FrameJson),
    SubmitMsg(FrameJson),
}

pub struct Chat {
    frame_model: FrameModel,
    wss: WebsocketService,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let wss = WebsocketService::new();

        // let message = WebSocketMessage {
        //     message_type: MsgTypes::Register,
        //     data: Some(username.to_string()),
        //     data_array: None,
        // };

        // if let Ok(_) = wss
        //     .tx
        //     .clone()
        //     .try_send(serde_json::to_string(&message).unwrap())
        // {
        //     log::debug!("message sent successfully");
        // }

        Self {
            frame_model: FrameModel::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::RecieveMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::RecieveMsg(f) => {
                self.frame_model.clone().push_front(f);
                true
            }
            Msg::SubmitMsg(f) => {
                // self.frame_model.clone().push_front(f);
                if let Err(e) = self.wss.tx.try_send(f) {
                    log::error!("error sending message: {}", e);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| {
            Msg::SubmitMsg(FrameJson {
                text: "a".to_string(),
                subtext: Some("b".to_string()),
            })
        });

        html! {
          <>
            <Frame fm={self.frame_model.clone()} />
            <FrameInput />
            <button onclick={submit}>{"Submit"}</button>
          </>
        }
    }
}
