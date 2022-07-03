use tokio::join;

use super::configuration::{build_tmdb_endpoint, MovieEndpoint, TMDbEndpoint};
use super::models::{ExtendedMovieDetails, MovieCredits, MovieDetails, MovieImages};

pub async fn get_movie_details(id: u32) -> Result<MovieDetails, reqwest::Error> {
    let movie_endpoint = MovieEndpoint::Details(id);
    let url = build_tmdb_endpoint(TMDbEndpoint::Movie(movie_endpoint));

    let response = reqwest::get(url).await?.json::<MovieDetails>().await?;

    Ok(response)
}

pub async fn get_movie_images(id: u32) -> Result<MovieImages, reqwest::Error> {
    let movie_endpoint = MovieEndpoint::Images(id);
    let url = build_tmdb_endpoint(TMDbEndpoint::Movie(movie_endpoint));

    let response = reqwest::get(url).await?.json::<MovieImages>().await?;

    Ok(response)
}

pub async fn get_movie_credits(id: u32) -> Result<MovieCredits, reqwest::Error> {
    let movie_endpoint = MovieEndpoint::Credits(id);
    let url = build_tmdb_endpoint(TMDbEndpoint::Movie(movie_endpoint));

    let response = reqwest::get(url).await?.json::<MovieCredits>().await?;

    Ok(response)
}

pub async fn get_movie_details_full(id: u32) -> ExtendedMovieDetails {
    let mut extended_details = ExtendedMovieDetails::new_empty();

    let (movie_details, movie_images, movie_credits) = join!(
        get_movie_details(id),
        get_movie_images(id),
        get_movie_credits(id)
    );

    if let Ok(details) = movie_details {
        extended_details.movie_details = Some(details);
    }

    if let Ok(images) = movie_images {
        extended_details.movie_images = Some(images);
    }

    if let Ok(credits) = movie_credits {
        extended_details.movie_credits = Some(credits);
    }

    extended_details
}
