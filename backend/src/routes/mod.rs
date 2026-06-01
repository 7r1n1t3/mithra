use actix_web::web;

pub mod api;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(api::configure));
}
