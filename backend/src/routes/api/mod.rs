use actix_web::web;

pub mod register;
pub mod signin;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/register").configure(register::configure));
    cfg.service(signin::post_signin);
}
