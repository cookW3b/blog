use diesel::prelude::*;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
}
