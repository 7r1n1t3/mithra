use actix_web::web;

pub mod code;
pub mod user;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(user::post_user);
}
