use super::*;

use futures::StreamExt;
use server::{
    request_message, request_message_sync, send_response_message, start_receive,
    RequestMessageResult, ResponseMessage, RxPasitos, TxHandle,
};

pub mod data;

common_macros::pasitos!(fut_queue, run_for;
    init {
        let start = std::time::Instant::now();
        let opt = (*OPT.read().unwrap()).clone().unwrap();
        match opt.cmd.as_ref().unwrap() {
            Command::Server {..} => {
                start_receive();
            },
        }
    }
    on_complete {
        info!(
            "{}, complete",
            arrange_millis::get(std::time::Instant::now().duration_since(start).as_millis()),
        );
        return Ok(());
    }
    on_next_end {
    }

    demoras {
        demora Forever({
            // duration: tokio::time::Duration,
        }) {
            common_macros::pasitos!(delay Forever { } for std::time::Duration::from_secs(1));
        }
    }

    pasos receive {
        max_at_once: 1;
        paso RequestMessage({
            rx: RxPasitos,
        }) -> ({
            res: RequestMessageResult,
        }) {
            let res = request_message(rx).await;
        } => sync {
            request_message_sync(res)?;
        }
    }

    pasos data {
        max_at_once: common_macros::settings!(data.max_at_once); // this limits access to expensive
                                                                 // internal resources
        paso Shorten({
            tx: TxHandle,
            payload: server::shorten::Payload,
        }) -> ( {
            res: pasitos::data::ShortenResult,
            tx: TxHandle,
        }) {
            let res = pasitos::data::shorten(payload).await;
        } => sync {
            pasitos::data::shorten_sync(res, tx)?;
        }

        paso Resolve({
            tx: TxHandle,
            payload: server::resolve::Payload,
        }) -> ( {
            res: pasitos::data::ResolveResult,
            tx: TxHandle,
        }) {
            let res = pasitos::data::resolve(payload).await;
        } => sync {
            pasitos::data::resolve_sync(res, tx)?;
        }

        paso Stat({
            tx: TxHandle,
            payload: server::stat::Payload,
        }) -> ( {
            res: pasitos::data::StatResult,
            tx: TxHandle,
        }) {
            let res = pasitos::data::stat(payload).await;
        } => sync {
            pasitos::data::stat_sync(res, tx)?;
        }
    }
);
