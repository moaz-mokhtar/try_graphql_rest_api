// ===

use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, PartialEq, Insertable, GraphQLObject)]
#[table_name = "users"]
/// User of the system.
pub struct User {
    /// User id of the user
    pub id: String,
    /// username of the user
    pub username: String,
    /// password of the user
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, PartialEq, Insertable, GraphQLObject)]
#[table_name = "team"]
/// Teams registered in the system.
pub struct Team {
    /// Team's id
    pub id: String,
    /// Name of the team
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, PartialEq, Insertable, GraphQLObject)]
#[table_name = "member"]
/// A member which related to a User and Team
pub struct Member {
    /// Memeber's id
    pub id: String,
    /// User id which related to this member
    pub user_id: String,
    /// team id which related to this member
    pub team_id: String,
}
