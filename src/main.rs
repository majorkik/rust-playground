#![feature(test)]

extern crate test;

mod katas;
mod tmdb_cli;

use crate::tmdb_cli::MovieDetails;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.themoviedb.org/3/movie/76341?api_key=6c6222ae2b7074b6260fcd8db18cb2a9&language=ru")
        .await?
        .json::<MovieDetails>()
        .await?;

    println!("{:#?}", resp);

    Ok(())
}
