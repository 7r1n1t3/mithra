use actix_files::NamedFile;
use actix_web::{HttpResponse, Responder, Result, get, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(healthcheck);
    cfg.service(get_index);
}

#[get("/")]
async fn get_index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
