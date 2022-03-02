use crate::{db::DbPool, entity::User};
use actix_web::{
    get,
    web::{self},
    Error, HttpResponse, Responder,
};

#[get("/health")]
/// Route to test API functionality without any communcation with DB.
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}

#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    // let connection = pool.get_ref().get().unwrap();
    let students_data = User::get_all(&pool).await.unwrap();
    // .map_err(|_| HttpResponse::InternalServerError().finish())?;

    let res = serde_json::to_string(&students_data).unwrap();

    Ok(HttpResponse::Ok().body(res))
}
