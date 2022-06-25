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
}

#[derive(Parser, Debug)]
pub struct MovieArgs {
    #[clap(value_parser)]
    pub id: u32,

    #[clap(short, long, arg_enum)]
    pub mode: MovieViewMode,
}

#[derive(ArgEnum, Clone, Debug)]
pub enum MovieViewMode {
    Default,
    Images,
    Credits,
    Full,
}

impl FromArgMatches for TypeSub {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some(("movie", args)) => Ok(Self::Movie(MovieArgs::from_arg_matches(args)?)),
            Some((_, _)) => Err(Error::raw(
                ErrorKind::UnrecognizedSubcommand,
                "Valid subcommands are `movie`",
            )),
            None => Err(Error::raw(
                ErrorKind::MissingSubcommand,
                "Valid subcommands are `movie`",
            )),
        }
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("movie", args)) => *self = Self::Movie(MovieArgs::from_arg_matches(args)?),
            Some((_, _)) => {
                return Err(Error::raw(
                    ErrorKind::UnrecognizedSubcommand,
                    "Valid subcommands are `movie`",
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
            .subcommand_required(true)
    }

    fn augment_subcommands_for_update(cmd: Command<'_>) -> Command<'_> {
        cmd.subcommand(MovieArgs::augment_args(Command::new("movie")))
            .subcommand_required(true)
    }

    fn has_subcommand(text: &str) -> bool {
        matches!(text, "movie")
    }
}
