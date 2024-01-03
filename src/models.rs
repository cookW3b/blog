use diesel::prelude::*;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub user_id: Uuid
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub user_id: Uuid
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub password: String,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub name: String,
    pub password: String
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct LoginUser {
    pub username: String,
    pub password: String
}
