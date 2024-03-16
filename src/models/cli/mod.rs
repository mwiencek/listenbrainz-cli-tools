use clap::{Parser, Subcommand};
use crate::tools::unlinked::unlinked_command;

/// Tools for Listenbrainz
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Tools with the unlinked listens
    Unlinked {
        /// Name of the user to fetch unlinked listen from
        #[arg(short, long)]
        username: String,

        /// Add links to search MusicBrainz for this recording
        #[arg(short, long)]
        musicbrainz: bool,
    },
}

impl Commands {
    pub fn run(&self) {
        match self {
            Commands::Unlinked { username, musicbrainz } => unlinked_command(username, *musicbrainz),
        }
    }
}
