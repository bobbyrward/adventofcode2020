#[macro_use]
mod args;
mod command;
mod point;

use anyhow::Result;
use clap::Clap;
use tracing_subscriber::FmtSubscriber;

use crate::command::Command;

#[allow(unused_imports)]
use crate::point::Point;

macro_rules! solution {
    ($($day:ident),+) => {
        $(
            mod $day;
        )+

        pub enum Day {
            $(
            #[allow(non_camel_case_types)]
            $day,
            )+
        }

        #[derive(Debug, Clap)]
        pub enum Solutions {
            $(
            #[allow(non_camel_case_types)]
            $day {
                #[clap(subcommand)]
                contents: crate::$day::Args,
            },
            )+
        }

        fn input(day: Day) -> &'static str {
            match day {
                $(Day::$day { .. } => include_str!(concat!("../../../inputs/", stringify!($day), ".txt")),)+
            }
        }

        // stringify!($day)
        impl Command for Solutions {
            fn execute(&self) -> anyhow::Result<String> {
                match self {
                    $(Self::$day { contents } => contents.execute(),)+
                }
            }
        }
    }
}

// NOTE: Each solution module must be added here
solution!(day01, day02, day03, day04, day06);

fn main() -> Result<()> {
    let args = args::Args::parse();

    FmtSubscriber::builder()
        .with_env_filter(args.env_filter())
        .init();

    let solution = args.command.execute()?;

    println!("Solution:\n{}", solution);

    Ok(())
}
