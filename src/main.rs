use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;
use net_gateway::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_log();
    let app_state = AppState::new().await;
    HttpServer::new(move || App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(net_gateway::endpoints::dashboards::overview::get_overview)
            .service(net_gateway::endpoints::charts::network_graph::get_network_graph)
            .service(net_gateway::endpoints::charts::bandwidth_per_endpoint::get_bandwidth_per_endpoint)
            .service(net_gateway::endpoints::charts::network_bandwidth::get_network_bandwidth)
        )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}
