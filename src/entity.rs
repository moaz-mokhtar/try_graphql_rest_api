// ===

use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, PartialEq, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, PartialEq, Insertable)]
#[table_name = "team"]
pub struct Team {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, PartialEq, Insertable)]
#[table_name = "member"]
pub struct Member {
    pub id: String,
    pub user_id: String,
    pub team_id: String,
}
