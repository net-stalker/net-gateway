use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;
use net_gateway::app_state::AppState;
use net_gateway::endpoints::chart::chart;
use net_gateway::endpoints::dashboard::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = AppState::new().await;
    HttpServer::new(move || App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(dashboard)
            .service(chart)
        )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
