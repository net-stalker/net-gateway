use actix_cors::Cors;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::http::header;
use actix_web::web;
use net_gateway::core::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    #[cfg(debug_assertions)]
    init_log();
    let app_state = AppState::new().await;
    HttpServer::new(move || App::new()
            .wrap(
                Cors::default()
                    // for profuction use this will be changed
                    .allowed_origin("http://localhost:4000")
                    .allowed_methods(vec!["GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            )
            .app_data(web::Data::new(app_state.clone()))
            .service(net_gateway::endpoints::dashboards::overview::endpoint::get_overview)
            .service(net_gateway::endpoints::charts::network_graph::endpoint::get_network_graph)
            .service(net_gateway::endpoints::charts::network_bandwidth_per_endpoint::endpoint::get_bandwidth_per_endpoint)
            .service(net_gateway::endpoints::charts::network_bandwidth::endpoint::get_network_bandwidth)
        )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(debug_assertions)]
fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}
