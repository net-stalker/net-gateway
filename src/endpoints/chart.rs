use actix_web::{HttpResponse, Responder, get, web};

#[get("/chart/{name}")]
#[allow(unused_variables)]
async fn chart(name: web::Path<String>, params: web::Query<serde_json::Value>) -> impl Responder {
    HttpResponse::Ok().json(params.into_inner())
}
