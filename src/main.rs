#![feature(test)]

extern crate test;

mod katas;
mod tmdb_cli;

use clap::clap_derive::ArgEnum;
use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args as _, Command, FromArgMatches, Parser, Subcommand};

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    type_subcommand: TypeSub,
}

#[derive(Debug)]
enum TypeSub {
    Movie(MovieArgs),
    TvShow(TvShowArgs),
}

#[derive(Parser, Debug)]
struct MovieArgs {
    #[clap(value_parser)]
    id: u32,

    #[clap(short, long, arg_enum)]
    mode: Option<MovieViewMode>,
}

#[derive(Parser, Debug)]
struct TvShowArgs {
    #[clap(value_parser)]
    id: u32,

    #[clap(short, long, arg_enum)]
    mode: Option<TVShowViewMode>,
}

#[derive(ArgEnum, Clone, Debug)]
enum MovieViewMode {
    Default,
    Images,
    Credits,
    Full,
}

#[derive(ArgEnum, Clone, Debug)]
enum TVShowViewMode {
    Default,
    Images,
    Credits,
    Seasons,
    Full,
}

impl FromArgMatches for TypeSub {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some(("movie", args)) => Ok(Self::Movie(MovieArgs::from_arg_matches(args)?)),
            Some(("tv", args)) => Ok(Self::TvShow(TvShowArgs::from_arg_matches(args)?)),
            Some((_, _)) => Err(Error::raw(
                ErrorKind::UnrecognizedSubcommand,
                "Valid subcommands are `movie` and `tv`",
            )),
            None => Err(Error::raw(
                ErrorKind::MissingSubcommand,
                "Valid subcommands are `movie` and `tv`",
            )),
        }
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("movie", args)) => *self = Self::Movie(MovieArgs::from_arg_matches(args)?),
            Some(("tv", args)) => *self = Self::TvShow(TvShowArgs::from_arg_matches(args)?),
            Some((_, _)) => {
                return Err(Error::raw(
                    ErrorKind::UnrecognizedSubcommand,
                    "Valid subcommands are `movie` and `tv`",
                ))
            }
            None => (),
        };

        Ok(())
    }
}

impl Subcommand for TypeSub {
    fn augment_subcommands(cmd: Command<'_>) -> Command<'_> {
        cmd.subcommand(MovieArgs::augment_args(Command::new("movie")))
            .subcommand(TvShowArgs::augment_args(Command::new("tv")))
            .subcommand_required(true)
    }

    fn augment_subcommands_for_update(cmd: Command<'_>) -> Command<'_> {
        cmd.subcommand(MovieArgs::augment_args(Command::new("movie")))
            .subcommand(TvShowArgs::augment_args(Command::new("tv")))
            .subcommand_required(true)
    }

    fn has_subcommand(text: &str) -> bool {
        matches!(text, "movie" | "tv")
    }
}

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

fn fetch_movie_details(id: u32, mode: Option<MovieViewMode>) {}

fn fetch_tv_details(id: u32, mode: Option<TVShowViewMode>) {}
