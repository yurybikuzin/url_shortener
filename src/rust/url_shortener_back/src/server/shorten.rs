use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    pub url: String,
}

#[post("/shorten")]
async fn handler(payload: web::Json<Payload>) -> actix_web::Result<impl Responder> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ResponseMessage>(1);
    (TX.write().unwrap())
        .as_mut()
        .unwrap()
        .try_send((tx, RequestMessage::Shorten(payload.into_inner())))
        .unwrap();
    if let Some(ResponseMessage::Shorten(res)) = rx.recv().await {
        res.map_err(|err| ServerError::Anyhow(err).into())
    } else {
        Err(ServerError::FailedToRecieveResponse("shorten").into())
    }
}
