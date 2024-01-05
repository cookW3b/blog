use diesel::{PgConnection, RunQueryDsl, QueryDsl, ExpressionMethods, QueryResult};
use uuid::Uuid;
use crate::models;
use crate::schema::*;

pub fn get_posts(conn: &mut PgConnection) -> Vec<models::Post> {
    let posts: Vec<models::Post> = posts::table.get_results(conn).expect("Cannot get a posts");
    posts
}

pub fn create_post(conn: &mut PgConnection, post: &models::Post) {
    diesel::insert_into(posts::table)
        .values(post)
        .execute(conn)
        .expect("Cannot create a new post");
}

pub fn create_user(conn: &mut PgConnection, user: &models::User) -> QueryResult<usize> {
    diesel::insert_into(users::table)
        .values(user)
        .execute(conn)
}

pub fn login_user(conn: &mut PgConnection, user: &models::LoginUser) -> QueryResult<models::User> {
    use crate::schema::users::dsl::*;
    users.filter(username.eq(&user.username))
        .first::<models::User>(conn)
}

pub fn create_comment(conn: &mut PgConnection, comment: &models::Comment) -> QueryResult<usize> {
    diesel::insert_into(comments::table)
        .values(comment)
        .execute(conn)
}

pub fn update_comment(conn: &mut PgConnection, updated_comment: &models::UpdateComment) -> QueryResult<usize> {
    use crate::schema::comments::dsl::*;
    diesel::update(comments.filter(id.eq(updated_comment.id)))
        .set(body.eq(&updated_comment.body))
        .execute(conn)
}

pub fn get_comment(conn: &mut PgConnection, comment_id: Uuid) -> QueryResult<models::Comment> {
    use crate::schema::comments::dsl::*;
    comments.filter(id.eq(comment_id))
        .first::<models::Comment>(conn)
}

pub fn get_post_comments(
    conn: &mut PgConnection,
    post_id: Uuid
) -> Result<Vec<models::Comment>, diesel::result::Error> {
    comments::table.filter(comments::post_id.eq(post_id))
        .load(conn)
}

pub fn delete_comment(
    conn: &mut PgConnection,
    comment_id: Uuid
) -> QueryResult<usize> {
    use crate::schema::comments::dsl::*;
    diesel::delete(comments.filter(id.eq(comment_id)))
        .execute(conn)

}
