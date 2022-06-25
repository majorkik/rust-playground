use std::fmt::Display;

use serde::Deserialize;

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
    pub release_date: String,
    pub revenue: i64,
    pub runtime: Option<i64>,
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

impl Display for MovieDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty_details: String = format!(
            "Title: {title}\nOriginal title: {original_title}\nOriginal language: {original_lang}\nGenres: {genres}\nHomepage: {homepage}\nTagline: {tagline}\nOverview: {overview}\nVote count: {vote_count}\nVote average: {vote_average}\nPopularity: {popularity}\n",
            title = self.title,
            original_title = self.original_title,
            original_lang = self.original_language,
            genres = self
                .genres
                .iter()
                .map(|item| item.name.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            homepage = self.homepage.as_deref().unwrap_or(""),
            tagline = self.tagline.as_deref().unwrap_or(""),
            overview = self.overview.as_deref().unwrap_or(""),
            vote_count = self.vote_count,
            vote_average = self.vote_average,
            popularity = self.popularity
        );

        write!(f, "Details:\n{}", pretty_details)
    }
}
