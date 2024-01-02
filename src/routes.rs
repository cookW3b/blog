use actix_web::{post, get, HttpResponse, Responder};
use diesel::prelude::*;


#[get("/posts")]
async fn get_posts() -> impl Responder {

}
