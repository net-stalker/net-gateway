use actix_web::{HttpResponse, Responder, get, web};

use crate::app_state::AppState;

#[get("/dashboard/{name}")]
#[allow(unused_variables)]
async fn dashboard(state: web::Data<AppState>, name: web::Path<String>, params: web::Query<serde_json::Value>) -> impl Responder {
    HttpResponse::Ok().json(params.into_inner())
}
