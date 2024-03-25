use actix_cors::Cors;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::http::header;
use actix_web::web;
use net_gateway::config::Config;
use net_core::set_ips;
use net_core::host::get_addr_from_host;

#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    #[cfg(debug_assertions)]
    init_log();
    let config = if cfg!(debug_assertions) {
        log::debug!("Running in debug mode");
        Config::builder().build().expect("read config error")
    } else {
        log::debug!("Running in release mode");
        let config_path = std::env::var("CONFIG_PATH").unwrap();
        let mut config = Config::new(&config_path).build().expect("read config error");
        set_ips!(config, get_addr_from_host, quin_reporter, quin_inserter);
        config
    };
    let config_clone = config.clone();
    HttpServer::new(move || App::new()
            .wrap(
                Cors::default()
                    // for profuction use this will be changed
                    // .allowed_origin(config_clone.allowed_origin.addr.as_str())
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
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
            .service(net_gateway::endpoints::charts::http_request_methods_distribution::endpoint::get_http_request_methods_distribution)
            .service(net_gateway::endpoints::filters::http_overview_filters::endpoint::get_http_overview_filters)
            .service(net_gateway::endpoints::dashboards::http_overview::endpoint::get_http_overview)
            .service(net_gateway::endpoints::pcap_files::pcap_files)
        )
        .bind(config.bind_address.addr)?
        .run()  
        .await
}

#[cfg(debug_assertions)]
fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}
