use clap::Clap;
use tracing_subscriber::EnvFilter;

use crate::Solutions;

#[derive(Debug, Clap)]
pub struct Args {
    #[clap(long, default_value = "warn")]
    pub logging_filter: String,

    #[clap(subcommand)]
    pub command: Solutions,
}

impl Args {
    pub fn env_filter(&self) -> EnvFilter {
        self.logging_filter.as_str().into()
    }
}
