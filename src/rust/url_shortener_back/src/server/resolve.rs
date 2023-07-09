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
    let referer = headers.get("Referer");
    debug!("ip: {ip:?}, referer: {referer:?}");
    payload.ip = ip.and_then(|val| {
        val.to_str().ok().and_then(|s| {
            IpAddr::from_str(s)
                .map_err(|err| error!("{}:{}: {err}", file!(), line!()))
                .ok()
        })
    });
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
        Err(ServerError::FailedToRecieveResponse("shorten").into())
    }
}
