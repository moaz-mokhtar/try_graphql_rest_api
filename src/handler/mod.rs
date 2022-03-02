pub mod gql;
pub mod rest;

use self::gql::create_schema;
use actix_web::web::{self, Data, ServiceConfig};

pub fn routes_config(config: &mut ServiceConfig) {
    let schema = Data::new(create_schema());
    config
        .app_data(schema)
        .service(rest::health)
        .service(rest::get_users)
        .service(web::resource("/graphql").route(web::post().to(gql::graphql)))
        .service(web::resource("/graphiql").route(web::get().to(gql::graphiql)));
}
