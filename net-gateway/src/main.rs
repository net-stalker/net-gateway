use actix_cors::Cors;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;
use net_gateway::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    #[cfg(debug_assertions)]
    init_log();
    let app_state = AppState::new().await;
    HttpServer::new(move || App::new()
            // TODO: Remove permissive CORS
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            )
            .app_data(web::Data::new(app_state.clone()))
            .service(net_gateway::endpoints::dashboards::overview::get_overview)
            .service(net_gateway::endpoints::charts::network_graph::endpoint::get_network_graph)
            .service(net_gateway::endpoints::charts::bandwidth_per_endpoint::get_bandwidth_per_endpoint)
            .service(net_gateway::endpoints::charts::network_bandwidth::get_network_bandwidth)
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
