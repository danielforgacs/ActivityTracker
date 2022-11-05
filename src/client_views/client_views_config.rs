use crate::prelude::*;

pub fn app_config(cfg: &mut ServiceConfig) {
    cfg.route("/", web::get().to(index::index_view));
}
