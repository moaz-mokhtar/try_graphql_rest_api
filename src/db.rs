use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use log::info;
use r2d2::Pool;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;


pub struct DbClientConn;
impl DbClientConn {
    pub fn get_pool_connection() -> DbPool {
        // it from the environment within this function
        dotenv().ok();
        let url = env::var("DATABASE_URL").expect("no DB URL");
        info!("DATABASE_URL: {url}");
        let migr = ConnectionManager::<PgConnection>::new(url);
        r2d2::Pool::builder()
            .build(migr)
            .expect("could not build connection pool")
    }
}
