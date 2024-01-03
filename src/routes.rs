use actix_web::{post, get, HttpResponse, Responder, web, error, HttpRequest};
use diesel::{prelude::*, r2d2::{self, ConnectionManager}};
use crate::{db, models};
use uuid::Uuid;
use bcrypt::{verify, hash};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/posts")]
pub async fn get_posts(db_pool: web::Data<DbPool>) -> impl Responder {
    let mut db_conn = db_pool.get().expect("Can't get DB connection from pool");
    let result = db::get_posts(&mut db_conn);

    HttpResponse::Ok().json(result)
}

#[post("/posts")]
pub async fn create_post(db_pool: web::Data<DbPool>, new_post: web::Json<models::NewPost>) -> impl Responder {
    let mut db_conn = db_pool.get().expect("Can't get DB connection from pool");

    let post = models::Post {
        title: new_post.0.title.to_owned(),
        body: new_post.0.body.to_owned(),
        user_id: new_post.0.user_id.to_owned(),
        id: Uuid::new_v4()
    };

    db::create_post(&mut db_conn, &post);

    HttpResponse::Created().json(post)
}

#[post("/users/signup")]
pub async fn create_user(db_pool: web::Data<DbPool>, new_user: web::Json<models::NewUser>) -> impl Responder {
    let mut db_conn = db_pool.get().expect("Can't get DB connection from pool");

    let user = models::User {
        id: Uuid::new_v4(),
        name: new_user.0.name.to_owned(),
        username: new_user.0.username.to_owned(),
        password: hash(new_user.0.password.to_owned(), 10).unwrap()
    };

    db::create_user(&mut db_conn, &user);

    HttpResponse::Created().json(user)
}

#[post("/users/login")]
pub async fn login_user(db_pool: web::Data<DbPool>, user: web::Json<models::LoginUser>) -> impl Responder {
    let mut db_conn = db_pool.get().expect("Can't get DB connection from pool");

    let existing_user = db::get_user(&mut db_conn, &user);

    if verify(&user.password, &existing_user.password).expect("Failed to verify password") {
        HttpResponse::Ok().json(existing_user)
    } else {
        HttpResponse::Unauthorized().body("Invalid crendentials")
    }
}
