use crate::prelude::*;

pub fn app_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/start", web::post().to(start))
            .route("/stop", web::post().to(stop))
            .route("/activities", web::get().to(get_activities_per_date))
            .route("/pretty", web::get().to(pretty)),
    );
}
