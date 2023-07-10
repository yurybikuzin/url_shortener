use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    pub ip: Option<IpAddr>,
    pub shortened: String,
}

#[get("/{shortened}")]
async fn handler(
    req: HttpRequest,
    mut payload: web::Path<Payload>,
) -> actix_web::Result<impl Responder> {
    let headers = req.headers();
    let ip = headers.get("X-Real-IP");
    payload.ip = ip.and_then(|val| {
        val.to_str().ok().and_then(|s| {
            IpAddr::from_str(s)
                .map_err(|err| error!("{}:{}: {err}", file!(), line!()))
                .ok()
        })
    });
    // We won't process the request here, as processing it can be expensive, so we'll send it to an internal queue, and after processing, we'll return the response to the user.
    // If we tried to process the request right here, it could result in a DDOS broadcast to internal resources
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ResponseMessage>(1);
    (TX.write().unwrap())
        .as_mut()
        .unwrap()
        .try_send((tx, RequestMessage::Resolve(payload.into_inner())))
        .unwrap();
    if let Some(ResponseMessage::Resolve(res)) = rx.recv().await {
        res.map_err(|err| ServerError::Anyhow(err).into())
            .and_then(|ret| {
                ret.ok_or_else(|| ServerError::NotFound.into())
                    .map(|url| Redirect::to(url).temporary())
            })
    } else {
        Err(ServerError::FailedToRecieveResponse("resolve").into())
    }
}
