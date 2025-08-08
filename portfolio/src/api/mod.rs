use actix_web::web;

pub mod echo;
pub mod helloworld;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(helloworld::index)
       .service(echo::echo);
}