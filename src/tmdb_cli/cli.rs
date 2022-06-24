use clap::clap_derive::ArgEnum;
use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args as _, Command, FromArgMatches, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub type_subcommand: TypeSub,
}

#[derive(Debug)]
pub enum TypeSub {
    Movie(MovieArgs),
    TvShow(TvShowArgs),
}

#[derive(Parser, Debug)]
pub struct MovieArgs {
    #[clap(value_parser)]
    pub id: u32,

    #[clap(short, long, arg_enum)]
    pub mode: Option<MovieViewMode>,
}

#[derive(Parser, Debug)]
pub struct TvShowArgs {
    #[clap(value_parser)]
    pub id: u32,

    #[clap(short, long, arg_enum)]
    pub mode: Option<TVShowViewMode>,
}

#[derive(ArgEnum, Clone, Debug)]
pub enum MovieViewMode {
    Default,
    Images,
    Credits,
    Full,
}

#[derive(ArgEnum, Clone, Debug)]
pub enum TVShowViewMode {
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
