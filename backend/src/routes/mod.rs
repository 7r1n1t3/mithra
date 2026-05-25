use actix_web::web;

pub mod auth;
pub mod root;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/auth").configure(auth::configure));
    cfg.service(web::scope("/").configure(root::configure));
}
