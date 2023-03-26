use std::env;
use dotenv::dotenv;
use actix_web::{get, web, App, HttpResponse, HttpServer, post};
use serde::{Deserialize, Serialize};

use movie_memo_db::schemas::movie;
use movie_memo_db::schemas::user::{User, UserError};
use movie_memo_db::schemas::user_movies::{UserMovie, UserMovieError, NewMovieParams};
use sqlx::types::Uuid;


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
    user_id: Uuid,
}

#[get("/user/{user_id}/movies")]
async fn fetch_url2(params: web::Path<GetUserMoviesParams>) -> HttpResponse {
    
    println!("[GET] /user/{}/movies", params.user_id);

    match UserMovie::get_by_user_id(&params.user_id).await {
        Ok(movies) => HttpResponse::Ok().json(movies),
        Err(err) => {
            let context = err.current_context().clone();

            match context {
                UserMovieError::AlreadyExists(msg) => HttpResponse::BadRequest().body(format!("{}", msg)),
                UserMovieError::InvalidArguments(msg) => HttpResponse::BadRequest().body(format!("{}", msg)),
                UserMovieError::NotFound(msg) => HttpResponse::NotFound().body(format!("{}", msg)),
                UserMovieError::SqlxError => HttpResponse::InternalServerError().body("Something went wrong. Try Again later")
            }
        },
    }
}

#[derive(Deserialize)]
struct CreateUserMoviesParams {
    user_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserMovieBody {
    movie_id: i32,
    title: String,
    seen: Option<bool>,
    watch_again: Option<bool>,
    rating: Option<i32>,
}

#[post("/user/{user_id}/movies")]
async fn create_user_movie(params: web::Path<CreateUserMoviesParams>, body: web::Json<NewMovieParams>) -> HttpResponse {
    
    println!("[GET] /user/{}/movies", params.user_id);

    let params = NewMovieParams {
        user_id: Some(params.user_id),
        movie_id: body.movie_id,
        title: body.title.clone(),
        rating: body.rating,
        seen: body.seen,
        watch_again: body.watch_again
    };

    match UserMovie::new(params).await {
        Ok(movies) => HttpResponse::Ok().json(movies),
        Err(err) => {
            let context = err.current_context().clone();

            match context {
                UserMovieError::AlreadyExists(msg) => HttpResponse::BadRequest().body(format!("{}", msg)),
                UserMovieError::InvalidArguments(msg) => HttpResponse::BadRequest().body(format!("{}", msg)),
                UserMovieError::NotFound(msg) => HttpResponse::NotFound().body(format!("{}", msg)),
                UserMovieError::SqlxError => HttpResponse::InternalServerError().body("Something went wrong. Try Again later")
            }
        },
    }
}

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
}

#[post("/user")]
async fn create_user(req_body: web::Json<CreateUserRequest>) -> HttpResponse {
    
    println!("[POST] /user - BODY: {}",&req_body.username);

    match User::new(&req_body.username).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            let context = err.current_context().clone();

            match context {
                UserError::AlreadyExists(msg) => HttpResponse::BadRequest().body(format!("{}", msg)),
                UserError::InvalidArguments(msg) => HttpResponse::BadRequest().body(format!("{}", msg)),
                UserError::NotFound(msg) => HttpResponse::NotFound().body(format!("{}", msg)),
                UserError::SqlxError => HttpResponse::InternalServerError().body("Something went wrong. Try Again later")
            }
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // This line loads the environment variables from the ".env" file.
    check_environment();
    let port = env::var("PORT").unwrap_or(String::from("8000"));
    let port_number = port.parse::<u16>().unwrap_or(8000);
    println!("Server running on port \x1b[32m{}\x1b[0m",port);
    HttpServer::new(|| {
        App::new()
            .service(fetch_url)
            .service(fetch_url2)
            .service(create_user)
            .service(create_user_movie)
    })
    .bind(("127.0.0.1", port_number))?
    .run()
    .await
}

fn check_environment() {
    if let Err(_) = env::var("TMDB_API_KEY_V3") {
        panic!("TMDB_API_KEY_V3 must be set in order to consume TMBD API.");
    }
    if let Err(_) = env::var("TMDB_URL_V3") {
        panic!("TMDB_URL_V3 must be set in order to consume TMBD API. (it's assumed the url does not finish with `/`)");
    }
    if let Err(_) = env::var("DATABASE_URL") {
        panic!("DATABASE_URL must be set in order to perform database operations. (format `postgres://{{DB_USER}}:{{DB_PASSWORD}}@{{DB_HOST}}/movie_memo`)");
    }
}