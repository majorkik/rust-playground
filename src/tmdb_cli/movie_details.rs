use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MovieDetails {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub budget: i64,
    pub genres: Vec<Genre>,
    pub homepage: Option<String>,
    pub id: i64,
    pub imdb_id: Option<String>,
    pub original_language: String,
    pub original_title: String,
    pub overview: Option<String>,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub production_companies: Vec<ProductionCompany>,
    pub production_countries: Vec<ProductionCountry>,
    pub release_date: String,
    pub revenue: i64,
    pub runtime: Option<i64>,
    pub spoken_languages: Vec<SpokenLanguage>,
    pub status: String,
    pub tagline: Option<String>,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ProductionCompany {
    pub id: i64,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: String,
}

#[derive(Debug, Deserialize)]
pub struct ProductionCountry {
    pub iso_3166_1: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct SpokenLanguage {
    pub iso_639_1: String,
    pub name: String,
}
