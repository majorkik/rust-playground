use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MovieImages {
    pub id: i64,
    pub backdrops: Vec<Backdrop>,
    pub posters: Vec<Poster>,
}

#[derive(Debug, Deserialize)]
pub struct Backdrop {
    pub aspect_ratio: f64,
    pub file_path: String,
    pub height: i64,
    pub vote_average: f32,
    pub vote_count: i64,
    pub width: i64,
}

#[derive(Debug, Deserialize)]
pub struct Poster {
    pub aspect_ratio: f64,
    pub file_path: String,
    pub height: i64,
    pub vote_average: f32,
    pub vote_count: i64,
    pub width: i64,
}

impl Display for MovieImages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_backdrops: Vec<String> = self
            .backdrops
            .iter()
            .map(|image| format!("https://image.tmdb.org/t/p/original{}", image.file_path))
            .collect();

        let formatted_posters: Vec<String> = self
            .posters
            .iter()
            .map(|image| format!("https://image.tmdb.org/t/p/original{}", image.file_path))
            .collect();

        write!(
            f,
            "Backdrops:\n{}\nPosters:\n{}",
            formatted_backdrops.join("\n"),
            formatted_posters.join("\n")
        )
    }
}
