use actix_web::{HttpResponse, Responder, get, web};

#[get("/dashboard/{name}")]
#[allow(unused_variables)]
async fn dashboard(name: web::Path<String>, params: web::Query<serde_json::Value>) -> impl Responder {
    HttpResponse::Ok().json(params.into_inner())
}
