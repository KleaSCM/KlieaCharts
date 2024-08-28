
//route handlers
use actix_web::{HttpResponse, Responder};

pub async fn get_data() -> impl Responder {
    HttpResponse::Ok().json("Welcome to KlieaCharts API!")
}
