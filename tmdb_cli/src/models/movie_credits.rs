use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MovieCredits {
    pub id: i64,
    pub cast: Vec<Cast>,
    pub crew: Vec<Crew>,
}

#[derive(Debug, Deserialize)]
pub struct Cast {
    pub adult: bool,
    pub gender: Option<i64>,
    pub id: i64,
    pub known_for_department: String,
    pub name: String,
    pub original_name: String,
    pub popularity: f64,
    pub profile_path: Option<String>,
    pub cast_id: i64,
    pub character: String,
    pub credit_id: String,
    pub order: i64,
}

#[derive(Debug, Deserialize)]
pub struct Crew {
    pub adult: bool,
    pub gender: Option<i64>,
    pub id: i64,
    pub known_for_department: String,
    pub name: String,
    pub original_name: String,
    pub popularity: f64,
    pub profile_path: Option<String>,
    pub credit_id: String,
    pub department: String,
    pub job: String,
}

impl Display for MovieCredits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_backdrops: Vec<String> = self
            .cast
            .iter()
            .map(|cast| {
                format!(
                    "Name: {} ({}) https://image.tmdb.org/t/p/original{}",
                    cast.original_name,
                    cast.character,
                    cast.profile_path
                        .to_owned()
                        .unwrap_or_else(|| "".to_string())
                )
            })
            .collect();

        let formatted_posters: Vec<String> = self
            .crew
            .iter()
            .map(|crew| {
                format!(
                    "Name: {}, department: {}, https://image.tmdb.org/t/p/original{}",
                    crew.original_name,
                    crew.department,
                    crew.profile_path
                        .to_owned()
                        .unwrap_or_else(|| "".to_string())
                )
            })
            .collect();

        write!(
            f,
            "Cast:\n{}\nCrew:\n{}",
            formatted_backdrops.join("\n"),
            formatted_posters.join("\n")
        )
    }
}
