use crate::entity::User;
use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

impl User {
    pub async fn get_all(connection: &PgConnection) -> Result<Vec<User>, DbError> {
        use crate::schema::users::dsl::users;
        let _users = users.load::<User>(connection)?;

        Ok(_users)
    }
}

// ====
// ====
// ====
// ====

// impl Student{

//     pub async fn get_all(connection:&DbPool)->Result<Vec<Student>,sqlx::Error>{
//         let query = format!("Select * from student");

//         let res = sqlx::query_as(query.as_str()).fetch_all(connection).await?;
//         Ok(res)
//     }

//     pub async fn get_by_id(connection:&DbPool,record_id: i32,)->Result<Student,sqlx::Error>{
//         let query = format!("Select * from student where id = {}",record_id);

//         match sqlx::query_as(query.as_str()).fetch_one(connection).await{
//             Ok(res)=>{
//                 println!("res {:?}",res);
//                 Ok(res)},
//             Err(e)=>{
//                 println!("error {:?}",e);
//                  Err(e)}
//         }
//        // Ok(res)
//     }

//     pub async fn archive_by_id(connection:&DbPool, record_id: i32,)->Result<bool,sqlx::Error>{
//         let query = format!("Update student SET archived = true where id = {}",record_id);

//         match sqlx::query(query.as_str()).execute(connection).await{
//             Ok(_res)=>{
//                // println!("res {:?}",res);
//                 Ok(true)},
//             Err(e)=>{
//                 println!("error {:?}",e);
//                  Err(e)}
//         }
//        // Ok(res)
//     }

//     pub async fn insert(new_request: NewStudent,connection:&DbPool)->Result<bool,sqlx::Error>{
//         let query = format!("insert into student(name,image_path) values('{}','{}') ",new_request.name.clone(),new_request.image_path.clone());
//         println!("query {}",query);
//         match sqlx::query(query.as_str()).execute(connection).await{
//             Ok(_res)=>{
//                 //res.get(0);
//                 //println!("res {}",_res);
//                 Ok(true)
//                 //Ok(Student{id:3,name:res.get(0),image_path:res.get(1)})},}
//             }
//             Err(e)=>{
//                 println!("error {:?}",e);
//                  Err(e)}
//         }
//     }

// }
