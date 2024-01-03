use actix_web::{HttpServer, App, Responder, HttpResponse, web, get};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use dotenvy::dotenv;

mod models;
mod schema;
mod routes;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not passed");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::new(manager).expect("Cannot create pool connection");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(routes::get_posts)
            .service(routes::create_post)
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}
