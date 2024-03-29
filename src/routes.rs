use actix_web::{post, get, HttpResponse, HttpRequest, Responder, web, delete};
use diesel::{prelude::*, r2d2::{self, ConnectionManager}};
use serde::Serialize;
use crate::{db, models, jwt};
use uuid::Uuid;
use bcrypt::{verify, hash};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize)]
struct ResponseMessage<'a> {
    message: &'a str
}

#[derive(Serialize)]
struct LoginResponse<'a> {
    token: &'a str
}

#[get("/posts")]
pub async fn get_posts(db_pool: web::Data<DbPool>) -> impl Responder {
    let mut db_conn = db_pool.get().expect("Can't get DB connection from pool");
    let result = db::get_posts(&mut db_conn);

    HttpResponse::Ok().json(result)
}

#[post("/posts")]
pub async fn create_post(
    req: HttpRequest,
    db_pool: web::Data<DbPool>,
    new_post: web::Json<models::NewPost>
) -> impl Responder {
    let mut db_conn = db_pool.get().expect("Can't get DB connection from pool");

    let token = req.headers().get("x-access-token");

    if token.is_none() {
        return HttpResponse::Unauthorized().json(ResponseMessage {
            message: "Failed to create a new post"
        });
    }

    let decoded_data = jwt::decode_jwt(token.unwrap().to_str().unwrap());

    if decoded_data.is_err() {
        return HttpResponse::BadRequest().json(ResponseMessage {
            message: "Failed to create a new post"
        });
    }

    let post = models::Post {
        title: new_post.0.title.to_owned(),
        body: new_post.0.body.to_owned(),
        user_id: decoded_data.unwrap().claims.user_id,
        id: Uuid::new_v4()
    };

    db::create_post(&mut db_conn, &post);

    HttpResponse::Created().json(post)
}

#[post("/users/signup")]
pub async fn create_user(
    db_pool: web::Data<DbPool>,
    new_user: web::Json<models::NewUser>
) -> impl Responder {
    let mut db_conn = db_pool.get().expect("Can't get DB connection from pool");

    let user = models::User {
        id: Uuid::new_v4(),
        name: new_user.0.name.to_owned(),
        username: new_user.0.username.to_owned(),
        password: hash(new_user.0.password.to_owned(), 10).unwrap()
    };

    let result = db::create_user(&mut db_conn, &user);

    if result.is_ok() {
        HttpResponse::Created().json(user)
    } else {
        HttpResponse::BadRequest().json(ResponseMessage {
            message: "Cannot create a new user"
        })
    }
}

#[get("/users/login")]
pub async fn login_user(
    db_pool: web::Data<DbPool>,
    user: web::Json<models::LoginUser>
) -> impl Responder {
    let mut db_conn = db_pool.get().expect("Can't get DB connection from pool");

    let existing_user = db::login_user(&mut db_conn, &user);

    if existing_user.is_err() {
        return HttpResponse::BadRequest().json(ResponseMessage {
            message: "User not found"
        });
    }

    let existing_user = existing_user.unwrap();

    if verify(&user.password, &existing_user.password).expect("Failed to verify password") {
        let token = jwt::create_jwt(existing_user.id).unwrap();
        HttpResponse::Ok().json(LoginResponse {
            token: token.as_str()
        })
    } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}

#[post("/comments")]
pub async fn create_comment(
    req: HttpRequest,
    db_pool: web::Data<DbPool>,
    new_comment: web::Json<models::NewComment>
) -> impl Responder {
    let mut db_conn = db_pool.get().expect("Can't get DB connection from pool");

    let token = req.headers().get("x-access-token");

    if token.is_none() {
        return HttpResponse::Unauthorized().json(ResponseMessage {
            message: "Failed to create a new comment"
        });
    }

    let decoded_data = jwt::decode_jwt(token.unwrap().to_str().unwrap());

    if decoded_data.is_err() {
        return HttpResponse::BadRequest().json(ResponseMessage {
            message: "Failed to create a new comment"
        });
    }

    let comment = models::Comment {
        user_id: decoded_data.unwrap().claims.user_id,
        post_id: new_comment.0.post_id,
        body: new_comment.0.body,
        id: Uuid::new_v4()
    };

    if let Ok(_) = db::create_comment(&mut db_conn, &comment) {
        return HttpResponse::Created().json(comment)
    } else {
        return HttpResponse::BadRequest().json(ResponseMessage {
            message: "Cannot create a new comment"
        })
    }
}

#[post["/comments/update"]]
pub async fn update_comment(
    req: HttpRequest,
    db_pool: web::Data<DbPool>,
    update_comment: web::Json<models::UpdateComment>
) -> impl Responder {
    let mut db_conn = db_pool.get().expect("Can't get DB connection from pool");

    let token = req.headers().get("x-access-token");

    if token.is_none() {
        return HttpResponse::Unauthorized().json(ResponseMessage {
            message: "Failed to update comment"
        });
    }

    let decoded_data = jwt::decode_jwt(token.unwrap().to_str().unwrap());

    if decoded_data.is_err() {
        return HttpResponse::BadRequest().json(ResponseMessage {
            message: "Failed to update comment"
        });
    }

    let comment = db::get_comment(&mut db_conn, update_comment.id);

    if comment.is_err() {
        return HttpResponse::NotFound().json(ResponseMessage {
            message: "Comment not found"
        })
    }

    let result = db::update_comment(&mut db_conn, &update_comment);

    if result.is_err() {
        return HttpResponse::BadRequest().json(ResponseMessage {
            message: "Failed to update comment"
        })
    }

    HttpResponse::Ok().json(comment.unwrap())
}

#[get("/posts/{post_id}/comments")]
pub async fn get_post_comments(
    db_pool: web::Data<DbPool>,
    post_id: web::Path<Uuid>
) -> impl Responder {
    let mut db_conn = db_pool.get().expect("Can't get DB connection from pool");

    let result = db::get_post_comments(&mut db_conn, *post_id);

    if result.is_err() {
        return HttpResponse::BadRequest().json(ResponseMessage {
            message: "Cannot get a post comments"
        });
    }

    HttpResponse::Ok().json(result.unwrap())
}

#[delete("/comments/{comment_id}")]
pub async fn delete_comment(
    req: HttpRequest,
    db_pool: web::Data<DbPool>,
    comment_id: web::Data<Uuid>
) -> impl Responder {
    let mut db_conn = db_pool.get().expect("Can't get DB connection from pool");

    let token = req.headers().get("x-access-token");

    if token.is_none() {
        return HttpResponse::Unauthorized().json(ResponseMessage {
            message: "Failed to delete comment"
        });
    }

    let decoded_data = jwt::decode_jwt(token.unwrap().to_str().unwrap());

    if decoded_data.is_err() {
        return HttpResponse::BadRequest().json(ResponseMessage {
            message: "Failed to delete comment"
        });
    }

    let comment = db::get_comment(&mut db_conn, **comment_id);

    if comment.is_err() {
        return HttpResponse::BadRequest().json(ResponseMessage {
            message: "Failed to delete comment"
        });
    }

    if comment.as_ref().unwrap().user_id != decoded_data.unwrap().claims.user_id {
        return HttpResponse::BadRequest().json(ResponseMessage {
            message: "Failed to delete comment"
        });
    }

    if let Err(_) = db::delete_comment(&mut db_conn, **comment_id) {
        return HttpResponse::BadRequest().json(ResponseMessage {
            message: "Failed to delete comment"
        });
    }

    HttpResponse::Ok().json(comment.unwrap())
}
