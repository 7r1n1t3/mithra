use actix_web::{HttpResponse, Responder, get, web};

pub mod api;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(api::configure));
    cfg.service(get_register);
    cfg.service(get_signin);
    cfg.service(health);
}

#[get("/register")]
async fn get_register() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/singin")]
async fn get_signin() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("ok")
}
