use std::env;
use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Debug, Deserialize, Serialize)]
pub struct Genre {
    id: i32,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductionCompany {
    id: i32,
    logo_path: Option<String>,
    name: String,
    origin_country: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductionCountry {
    iso_3166_1: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpokenLanguage {
    english_name: String,
    iso_639_1: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Movie {
    adult: bool,
    backdrop_path: Option<String>,
    belongs_to_collection: Option<String>,
    budget: Option<i32>,
    genres: Option<Vec<Genre>>,
    genre_ids: Option<Vec<i32>>,
    homepage: Option<String>,
    id: i32,
    imdb_id: Option<String>,
    original_language: String,
    original_title: String,
    overview: String,
    popularity: f64,
    poster_path: Option<String>,
    production_companies: Option<Vec<ProductionCompany>>,
    production_countries: Option<Vec<ProductionCountry>>,
    release_date: String,
    revenue: Option<i32>,
    runtime: Option<i32>,
    spoken_languages: Option<Vec<SpokenLanguage>>,
    status: Option<String>,
    tagline: Option<String>,
    title: String,
    video: bool,
    vote_average: f64,
    vote_count: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchMovieResult {
    page: i32,
    total_results: i32,
    total_pages: i32,
    results: Vec<Movie>,
}

pub async fn fetch_movie_info(query: &str) -> Result<SearchMovieResult, reqwest::Error> {
    let api_key = env::var("API_KEY_V3").expect("Expected API key in API_KEY_V3 env var");
    let url = env::var("API_URL_V3").expect("Expected API url in API_URL_V3 env var");

    let params = [("api_key", api_key.as_str()),("query",query)];
    // If we use reqwest clients, the http calls are non blocking
    let client = reqwest::Client::new();
    let a = client.get(format!("{}/search/movie",url)).query(&params).build().unwrap();
    let urll = a.url();
    println!("send request to \"{}\"",urll);
    let response = client.get(format!("{}/search/movie",url)).query(&params).send().await?;
    let movies: SearchMovieResult = response.json().await?;

    Ok(movies)
}