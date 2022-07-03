use std::fmt::Display;

use super::{MovieCredits, MovieDetails, MovieImages};

pub struct ExtendedMovieDetails {
    pub movie_details: Option<MovieDetails>,
    pub movie_images: Option<MovieImages>,
    pub movie_credits: Option<MovieCredits>,
}

impl ExtendedMovieDetails {
    pub fn new_empty() -> ExtendedMovieDetails {
        ExtendedMovieDetails {
            movie_details: None,
            movie_images: None,
            movie_credits: None,
        }
    }
}

impl Display for ExtendedMovieDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let details_fmt = if let Some(details) = &self.movie_details {
            details.to_string()
        } else {
            String::from("Could not get info about the movie.")
        };

        let images_fmt = if let Some(images) = &self.movie_images {
            images.to_string()
        } else {
            String::from("Couldn't get a list of movie images.")
        };

        let credits_fmt = if let Some(credits) = &self.movie_credits {
            credits.to_string()
        } else {
            String::from("Couldn't get info about the actors and crew.")
        };

        write!(f, "{}\n{}\n{}", details_fmt, images_fmt, credits_fmt)
    }
}
