use actix_web::web;

pub mod register;
pub mod signin;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(register::post_register);
    cfg.service(signin::post_signin);
}
