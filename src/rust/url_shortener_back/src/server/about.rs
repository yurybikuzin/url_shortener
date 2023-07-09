use super::*;

#[get("/about")]
async fn handler() -> impl Responder {
    actix_web::HttpResponse::Ok().body(format!(
        "{} {} {}\n",
        *OP_MODE.read().unwrap(),
        built_info::PKG_NAME,
        built_info::PKG_VERSION,
    ))
}
