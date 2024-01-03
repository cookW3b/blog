use diesel::{PgConnection, RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::models;
use crate::schema::{posts, users};

pub fn get_posts(conn: &mut PgConnection) -> Vec<models::Post> {
    let posts: Vec<models::Post> = posts::table.get_results(conn).expect("Cannot get a posts");
    posts
}

pub fn create_post(conn: &mut PgConnection, post: &models::Post) {
    diesel::insert_into(posts::table)
        .values(post)
        .execute(conn)
        .expect("Cannot create new post");
}

pub fn create_user(conn: &mut PgConnection, user: &models::User) {
    diesel::insert_into(users::table)
        .values(user)
        .execute(conn)
        .expect("Cannot create new user");
}

pub fn get_user(conn: &mut PgConnection, user: &models::LoginUser) -> models::User {
    let result = users::table.filter(users::username.eq(&user.username))
        .first(conn)
        .expect("User not found");

    result
}
