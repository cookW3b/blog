use diesel::{PgConnection, RunQueryDsl};

use crate::{models, schema};
use actix_web::web;

pub fn get_posts(conn: &mut PgConnection) -> Vec<models::Post> {
    use crate::schema::posts;
    let posts: Vec<models::Post> = posts::table.get_results(conn).expect("Cannot get a posts");
    posts
}

pub fn create_post(conn: &mut PgConnection, post: &models::Post) {
    diesel::insert_into(schema::posts::table)
        .values(post)
        .execute(conn)
        .expect("Cannot create new post");
}

pub fn create_user(conn: &mut PgConnection, user: &models::User) {
    diesel::insert_into(schema::users::table)
        .values(user)
        .execute(conn)
        .expect("Cannot create new user");
}
