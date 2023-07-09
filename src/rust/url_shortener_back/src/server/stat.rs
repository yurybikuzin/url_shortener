use super::*;

pub type Payload = super::resolve::Payload;
#[get("/stat/{shortened}")]
async fn handler(payload: web::Path<Payload>) -> actix_web::Result<impl Responder> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ResponseMessage>(1);
    (TX.write().unwrap())
        .as_mut()
        .unwrap()
        .try_send((tx, RequestMessage::Stat(payload.into_inner())))
        .unwrap();
    if let Some(ResponseMessage::Stat(res)) = rx.recv().await {
        res.map_err(|err| ServerError::Anyhow(err).into())
            .and_then(|ret| {
                ret.ok_or_else(|| ServerError::NotFound.into())
                    .map(web::Json)
            })
    } else {
        Err(ServerError::FailedToRecieveResponse("shorten").into())
    }
}
