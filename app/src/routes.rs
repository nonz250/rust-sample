use actix_web::{web};
use crate::controllers::posts::index;
use crate::controllers::posts::store;
use crate::controllers::posts::update;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index));
    cfg.route("/posts/store", web::get().to(store::index));
    cfg.route("/posts/update/{id}", web::get().to(update::index));
}
