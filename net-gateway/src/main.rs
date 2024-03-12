use actix_cors::Cors;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::http::header;
use actix_web::web;
use net_gateway::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    #[cfg(debug_assertions)]
    init_log();
    let config = Config::builder().build().expect("read config error");
    let config_clone = config.clone();
    HttpServer::new(move || App::new()
            .wrap(
                Cors::default()
                    // for profuction use this will be changed
                    .allowed_origin(config_clone.allowed_origin.addr.as_str())
                    .allowed_methods(vec!["GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            )
            .app_data(web::Data::new(config_clone.clone()))
            .service(net_gateway::endpoints::dashboards::network_overview::endpoint::get_network_overview)
            .service(net_gateway::endpoints::charts::network_graph::endpoint::get_network_graph)
            .service(net_gateway::endpoints::charts::network_bandwidth_per_endpoint::endpoint::get_bandwidth_per_endpoint)
            .service(net_gateway::endpoints::charts::network_bandwidth_per_protocol::endpoint::get_network_bandwidth_per_protocol)
            .service(net_gateway::endpoints::charts::network_bandwidth::endpoint::get_network_bandwidth)
            .service(net_gateway::endpoints::charts::total_http_requests::endpoint::get_total_http_requests)
            .service(net_gateway::endpoints::charts::http_responses_distribution::endpoint::get_http_responses_distribution)
            .service(net_gateway::endpoints::charts::http_clients::endpoint::get_http_clients)
            .service(net_gateway::endpoints::charts::http_responses::endpoint::get_http_responses)
            .service(net_gateway::endpoints::charts::http_request_methods_dist::endpoint::get_http_request_methods_dist)
        )
        .bind(config.bind_addres.addr.as_str())?
        .run()  
        .await
}

#[cfg(debug_assertions)]
fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}
