use super::*;

mod about;
mod error;
pub mod resolve;
pub mod shorten;
pub mod stat;

// use common_macros::pasitos;
use error::*;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::RwLock;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use actix_web::{
    get, post,
    web::{self, Redirect, ServiceConfig},
    HttpRequest, Responder,
};

// https://stackoverflow.com/questions/57457202/how-can-i-return-a-configured-app-in-actix-web
pub fn config_app() -> Box<dyn Fn(&mut ServiceConfig)> {
    Box::new(move |cfg: &mut ServiceConfig| {
        cfg.service(about::handler);
        cfg.service(shorten::handler);
        cfg.service(resolve::handler);
        cfg.service(stat::handler);
    })
}

#[derive(Debug)]
pub enum RequestMessage {
    Shorten(shorten::Payload),
    Resolve(resolve::Payload),
    Stat(stat::Payload),
}

#[derive(Debug)]
pub enum ResponseMessage {
    Shorten(Result<String>),
    Resolve(Result<Option<String>>),
    Stat(Result<Option<crate::pasitos::data::Shortened>>),
}

pub fn process_request_message(request_message: RequestMessage, tx: TxHandle) {
    let ret = match request_message {
        RequestMessage::Shorten(payload) => {
            common_macros::pasitos!(data push_back Shorten { tx, payload });
            None
        }
        RequestMessage::Resolve(payload) => {
            common_macros::pasitos!(data push_back Resolve { tx, payload });
            None
        }
        RequestMessage::Stat(payload) => {
            common_macros::pasitos!(data push_back Stat { tx, payload });
            None
        }
    };
    if let Some((tx, message)) = ret {
        send_response_message(message, tx);
    }
}

pub type TxPasitos = Sender<(Sender<ResponseMessage>, RequestMessage)>;

pub type RxPasitos = Receiver<(Sender<ResponseMessage>, RequestMessage)>;
pub type TxHandle = Sender<ResponseMessage>;
pub const CHANNEL_LEN: usize = 10000;

lazy_static::lazy_static! {
    pub static ref TX: RwLock<Option<TxPasitos>> = RwLock::new(None);
}

pub fn start_receive() {
    let (tx, rx) = channel::<(Sender<ResponseMessage>, RequestMessage)>(CHANNEL_LEN);
    *TX.write().unwrap() = Some(tx);
    common_macros::pasitos!(receive push_back RequestMessage { rx });
}

pub fn send_response_message(response_message: ResponseMessage, tx: TxHandle) {
    if let Err(err) = tx.try_send(response_message) {
        error!("{}:{}: failed to send response: {err} ", file!(), line!());
    }
}

pub struct RequestMessageResult(pub (TxHandle, RequestMessage), pub RxPasitos);
pub async fn request_message(mut rx: RxPasitos) -> RequestMessageResult {
    RequestMessageResult(rx.recv().await.unwrap(), rx)
}
pub fn request_message_sync(
    RequestMessageResult((tx, request_message), rx): RequestMessageResult,
) -> Result<()> {
    process_request_message(request_message, tx);
    common_macros::pasitos!(receive push_back RequestMessage { rx });
    Ok(())
}
