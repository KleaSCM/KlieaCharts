use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/data", web::get().to(crate::handlers::get_data))
    );
}
