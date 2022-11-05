use crate::prelude::*;

pub fn app_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("start", web::post().to(start))
            .route("stop", web::post().to(stop))
            .route("times", web::get().to(times))
            .route("pretty", web::get().to(pretty)),
    );
}
