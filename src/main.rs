use actix_web::App;
use actix_web::HttpServer;
use net_gateway::endpoints::chart::chart;
use net_gateway::endpoints::dashboard::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
            .service(dashboard)
            .service(chart)
        )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
