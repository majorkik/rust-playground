#![feature(test)]

extern crate test;

mod katas;
mod tmdb_cli;

use clap::Parser;
use tmdb_cli::cli::{Args, MovieViewMode, TVShowViewMode, TypeSub};

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);

    match args.type_subcommand {
        TypeSub::Movie(args) => {
            println!("id: {}", args.id);

            fetch_movie_details(args.id, args.mode);
        }
        TypeSub::TvShow(args) => {
            fetch_tv_details(args.id, args.mode);
        }
    };
}

fn fetch_movie_details(id: u32, mode: Option<MovieViewMode>) {
    todo!();
}

fn fetch_tv_details(id: u32, mode: Option<TVShowViewMode>) {
    todo!();
}
