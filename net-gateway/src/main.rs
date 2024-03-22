use core::panic;
use std::net::Ipv4Addr;
use std::str::FromStr;

use actix_cors::Cors;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::http::header;
use actix_web::web;
use net_gateway::config::Config;
use tokio::process::Command;

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
        let reporter_ip = get_addr_from_host(&config.quin_reporter.host_name).await;
        if let Err(e) = Ipv4Addr::from_str(&reporter_ip) {
            panic!("Error parsing reporter ip: {}", e);
        }
        config.quin_reporter.addr = format!("{}:{}", reporter_ip, &config.quin_reporter.port);
        let inserter_ip = get_addr_from_host(&config.quin_inserter.host_name).await;
        if let Err(e) = Ipv4Addr::from_str(&inserter_ip) {
            panic!("Error parsing inserter ip: {}", e);
        }
        config.quin_inserter.addr = format!("{}:{}", inserter_ip, &config.quin_inserter.port);
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
        .bind("0.0.0.0:8080")?
        .run()  
        .await
}

#[cfg(debug_assertions)]
fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}

async fn get_addr_from_host(host_name: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("getent hosts {} | awk '{{ print $1 }}'", host_name))
        .output()
        .await
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
