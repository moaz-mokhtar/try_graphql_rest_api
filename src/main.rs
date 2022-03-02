use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use log::info;
use try_graphql_rest_api::{db::DbClientConn, handler, utils};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    info!("Welcome by Moaz bin Mohamed Mokhtar");
    utils::initiate_logging();

    let pool = DbClientConn::get_pool_connection();
    let data = Data::new(pool);
    let address = "127.0.0.1:8080";

    info!("Address: {address}");
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(handler::routes_config)
    })
    .bind(address)?
    .run()
    .await
}
