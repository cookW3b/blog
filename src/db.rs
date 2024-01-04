use diesel::{PgConnection, RunQueryDsl, QueryDsl, ExpressionMethods, QueryResult};
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

pub fn create_user(conn: &mut PgConnection, user: &models::User) -> QueryResult<usize> {
    diesel::insert_into(users::table)
        .values(user)
        .execute(conn)
}

pub fn get_user(conn: &mut PgConnection, user: &models::LoginUser) -> QueryResult<models::User> {
    use crate::schema::users::dsl::*;
    users.filter(username.eq(&user.username))
        .first::<models::User>(conn)
}
