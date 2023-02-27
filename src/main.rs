use std::env;
use dotenv::dotenv;
use actix_web::{get, web, App, HttpResponse, HttpServer, post};
use serde::Deserialize;

use movie_memo_db::connection::get_connection; // TODO: move schemas and db related stuff to "db" crates
use movie_memo_db::schemas::movie; // TODO: move schemas and db related stuff to "db" crates
use movie_memo_db::schemas::user_movies::UserMovie; // TODO: move schemas and db related stuff to "db" crates


#[derive(Deserialize)]
struct FetchRequest {
    search: String,
}

#[get("/movie")]
async fn fetch_url(req_body: web::Query<FetchRequest>) -> HttpResponse {

    println!("[GET] /fetch - QUERY: {}",&req_body.search);

    match movie::fetch_movie_info(&req_body.search).await {
        Ok(movie) => HttpResponse::Ok().json(movie),
        Err(err) => {
            println!("Unhandled Error: {:?}",err);
            HttpResponse::NotFound().body("Not found")
        },
    }
}

#[derive(Deserialize)]
struct GetUserMoviesParams {
    user_id: i32,
}

#[get("/user/{user_id}/movies")]
async fn fetch_url2(params: web::Path<GetUserMoviesParams>) -> HttpResponse {
    
    println!("[GET] /user/{}/movies", params.user_id);

    let user_movies = UserMovie::get_by_user_id(params.user_id);
    
    HttpResponse::Ok().json(user_movies)
}

#[derive(Deserialize)]
struct CreateUserRequest {
    search: String,
}

#[post("/user")]
async fn create_user(req_body: web::Json<CreateUserRequest>) -> HttpResponse {
    
    println!("[POST] /user - BODY: {}",&req_body.search);

    match movie::fetch_movie_info(&req_body.search).await {
        Ok(movie) => HttpResponse::Ok().json(movie),
        Err(err) => {
            println!("Unhandled Error: {:?}",err);
            HttpResponse::NotFound().body("Not found")
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // This line loads the environment variables from the ".env" file.
    let port = env::var("PORT").unwrap_or(String::from("8000"));
    let port_number = port.parse::<u16>().unwrap_or(8000);
    println!("Server running on port {}",port);
    HttpServer::new(|| {
        App::new()
            .service(fetch_url)
            .service(fetch_url2)
    })
    .bind(("127.0.0.1", port_number))?
    .run()
    .await
}