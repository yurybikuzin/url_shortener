use super::*;

pub type Payload = super::resolve::Payload;
#[get("/stat/{shortened}")]
async fn handler(payload: web::Path<Payload>) -> actix_web::Result<impl Responder> {
    // We won't process the request here, as processing it can be expensive, so we'll send it to an internal queue, and after processing, we'll return the response to the user.
    // If we tried to process the request right here, it could result in a DDOS broadcast to internal resources
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
        Err(ServerError::FailedToRecieveResponse("stat").into())
    }
}
