#![feature(test)]

extern crate test;

mod katas;
mod tmdb_cli;

use clap::Parser;

use tmdb_cli::cli::{Args, MovieViewMode, TypeSub};

use crate::tmdb_cli::api::{
    get_movie_credits, get_movie_details, get_movie_details_full, get_movie_images,
};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.type_subcommand {
        TypeSub::Movie(args) => match args.mode {
            MovieViewMode::Default => {
                match get_movie_details(args.id).await {
                    Ok(details) => println!("{}", details),
                    Err(error) => println!("Something went wrong: {}", error),
                };
            }
            MovieViewMode::Images => {
                match get_movie_images(args.id).await {
                    Ok(images) => println!("{}", images),
                    Err(error) => println!("Something went wrong: {}", error),
                };
            }
            MovieViewMode::Credits => {
                match get_movie_credits(args.id).await {
                    Ok(credits) => println!("{}", credits),
                    Err(error) => println!("Something went wrong: {}", error),
                };
            }
            MovieViewMode::Full => {
                let result = get_movie_details_full(args.id).await;
                println!("{}", result);
            }
        },
    };
}
