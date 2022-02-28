use actix_web::{
    get,
    web::{self, ServiceConfig},
    Error, HttpResponse, Responder,
};

use crate::{db::DbPool, entity::User};
// use crate::db::DbPool;
// use crate::entity::*;

pub fn routes_config(config: &mut ServiceConfig) {
    config
    .service(health)
    .service(get_users)
    // .service(handler::health)
    // ===
    // .route("/student/{id}", web::get().to(get_by_id))
    // .route("/student/add", web::post().to(add))
    // .route("/student/archive/{id}", web::delete().to(archive_by_id))
    ;
}

// =====
// =====
// =====

#[get("/health")]
/// Route to test API functionality without any communcation with DB.
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}

#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let connection = pool.get_ref().get().unwrap();
    let students_data = User::get_all(&connection).await.unwrap();
    // .map_err(|_| HttpResponse::InternalServerError().finish())?;

    let res = serde_json::to_string(&students_data).unwrap();

    Ok(HttpResponse::Ok().body(res))
}

// pub async fn add(
//     pool: web::Data<DbPool>,
//     new_request: web::Json<NewStudent>,
// ) -> Result<HttpResponse, Error> {
//     let connection = pool.get_ref();
//     let new_student = NewStudent {
//         name: new_request.name.clone(),
//         image_path: new_request.image_path.clone(),
//     };
//     println!("inside add");
//     //let new_student= new_student.as_ref();
//     let _students_data = Student::insert(new_student, connection)
//         .await
//         .map_err(|_| HttpResponse::InternalServerError())?;

//     println!("inside add2");
//     Ok(HttpResponse::Ok().body("success"))
// }

// pub async fn get_by_id(
//     pool: web::Data<DbPool>,
//     record_id: web::Path<i32>,
// ) -> Result<HttpResponse, Error> {
//     let connection = pool.get_ref();
//     let record_id = record_id.into_inner();
//     let record_data = Student::get_by_id(connection, record_id)
//         .await
//         .map_err(|_| HttpResponse::InternalServerError().finish())?;

//     let body = serde_json::to_string(&record_data).unwrap(); //hb.render("student", &student_data).unwrap();

//     Ok(HttpResponse::Ok().body(body))
// }

// pub async fn archive_by_id(
//     pool: web::Data<DbPool>,
//     record_id: web::Path<i32>,
// ) -> Result<HttpResponse, Error> {
//     let connection = pool.get_ref();
//     let record_id = record_id.into_inner();
//     let record_data = Student::archive_by_id(connection, record_id)
//         .await
//         .map_err(|_| HttpResponse::InternalServerError().finish())?;

//     let body = serde_json::to_string(&record_data).unwrap();
//     Ok(HttpResponse::Ok().body(body))
// }
