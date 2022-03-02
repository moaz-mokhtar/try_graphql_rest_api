use crate::db::DbPool;
use crate::entity::User;
use actix_web::{web, HttpResponse};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use juniper::{EmptyMutation, EmptySubscription, FieldError};

#[derive(Clone)]
pub struct Context {
    pub pool: DbPool,
}

/// Context Marker
impl juniper::Context for Context {}

pub async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql", Some("ws://localhost:8080/subscriptions"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    data: web::Json<GraphQLRequest>,
    schema: web::Data<Schema>,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let context = Context {
        pool: pool.get_ref().to_owned(),
    };
    let res = data.execute(&schema, &context).await;

    HttpResponse::Ok().json(res)
}

// =======================
// =======================

pub struct Query {}

#[juniper::graphql_object(
    Context = Context,
)]
impl Query {
    pub async fn api_version() -> &str {
        "1.0"
    }

    pub async fn users(context: &Context) -> Result<Vec<User>, FieldError> {
        // let connection = context.pool.get().unwrap();
        let users = User::get_all(&context.pool).await.unwrap();
        Ok(users)
    }
}

// =======================
// =======================

pub type Schema =
    juniper::RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new(), EmptySubscription::new())
}
