use actix_web::{
    error,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use log::info;
use step_by_tech::{db::DbClientConn, handler, utils};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    info!("Welcome by Moaz bin Mohamed Mokhtar");
    utils::initiate_logging();

    // let address = std::env::var("ADDRESS").expect("Missed 'ADDRESS' environment variable");
    // info!("address: {}", address);
    // let server = Server::http(address.as_str()).expect("Failed to start server.");
    // info!("Tiny server started");

    let pool = DbClientConn::get_pool_connection();
    let data = Data::new(pool);

    let json_config = web::JsonConfig::default().error_handler(|err, _req| {
        // create custom error response
        error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
    });
    let address = "127.0.0.1:8080";

    info!("Address: {address}");
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(handler::routes_config)
            .app_data(json_config.clone())
    })
    .bind(address)?
    .run()
    .await
}
